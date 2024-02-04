use std::{fs::read_to_string, path::PathBuf};

use anyhow::{anyhow, Result};
use swc_common::{
    errors::{ColorConfig, Handler},
    input::StringInput,
    sync::Lrc,
    FileName, SourceMap,
};
use swc_ecma_ast::{
    BlockStmt, ClassDecl, ClassMember, Decl, DefaultDecl, Expr, ImportDecl, ImportSpecifier, Lit,
    ModuleDecl, ModuleExportName, ModuleItem, OptChainBase, Pat, Prop, PropOrSpread, Stmt,
    SuperProp, VarDeclarator,
};
use swc_ecma_parser::TsConfig;
use swc_ecma_parser::{lexer::Lexer, Parser, Syntax};

pub fn parse_source(path: &PathBuf) -> Result<Vec<String>> {
    let contents = read_to_string(path).unwrap();

    let source_map: Lrc<SourceMap> = Default::default();
    let handler =
        Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(source_map.clone()));

    let file_path = path.to_str().ok_or(anyhow!(""))?.to_owned();
    let source_file = source_map.new_source_file(FileName::Custom(file_path), contents);
    let ts_config: TsConfig = TsConfig {
        tsx: false,
        decorators: true,
        dts: false,
        no_early_errors: false,
        disallow_ambiguous_jsx_like: false,
    };
    let lexer = Lexer::new(
        Syntax::Typescript(ts_config),
        Default::default(),
        StringInput::from(&*source_file),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    let module = parser
        .parse_module()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .map_err(|_| anyhow!("failed to parse module"))?;

    let mut sqls = vec![];

    let import_alias = module
        .body
        .iter()
        .filter_map(|line| match line {
            ModuleItem::ModuleDecl(ModuleDecl::Import(module_import_decl)) => {
                Some(module_import_decl)
            }
            _ => None,
        })
        .find_map(|import_decl| find_sqlx_import_alias(import_decl, "ts-sqlx", "sqlx"))
        .unwrap_or_else(|| "sqlx".to_string());

    for item in &module.body {
        match item {
            ModuleItem::Stmt(stmt) => {
                recurse_and_find_sql(&mut sqls, stmt, &import_alias)?;
            }
            ModuleItem::ModuleDecl(decl) => match decl {
                ModuleDecl::Import(_) => {}
                ModuleDecl::ExportDecl(export_decl) => {
                    let decl = export_decl.decl.clone();
                    process_decl(&mut sqls, &decl, &import_alias)?;
                }
                ModuleDecl::ExportNamed(_) => {}
                ModuleDecl::ExportDefaultDecl(export_default_decl) => {
                    let decl = export_default_decl.decl.clone();
                    process_default_decl(&mut sqls, &decl, &import_alias)?;
                }
                ModuleDecl::ExportDefaultExpr(export_default_expr) => {
                    let expr = export_default_expr.expr.clone();
                    get_sql_from_expr(&mut sqls, &expr, &import_alias)
                }
                ModuleDecl::ExportAll(_) => {}
                ModuleDecl::TsImportEquals(_) => {}
                ModuleDecl::TsExportAssignment(_) => {}
                ModuleDecl::TsNamespaceExport(_) => {}
            },
        }
    }

    Ok(sqls)
}

pub fn find_sqlx_import_alias(
    import_decl: &ImportDecl,
    package: &str,
    default_import: &str,
) -> Option<String> {
    if import_decl.src.value.to_string() == package {
        for specifier in &import_decl.specifiers {
            if let ImportSpecifier::Named(import_named_specifier) = specifier {
                if let Some(imported) = &import_named_specifier.imported {
                    match imported {
                        ModuleExportName::Ident(ident) => {
                            if ident.sym == default_import {
                                return Some(import_named_specifier.local.sym.to_string());
                            }
                        }
                        _ => continue,
                    }
                } else if &import_named_specifier.local.sym.to_string() == default_import {
                    return Some(default_import.to_string());
                }
            }
        }
    }
    None
}

fn recurse_and_find_sql(sqls: &mut Vec<String>, stmt: &Stmt, import_alias: &String) -> Result<()> {
    match stmt {
        Stmt::Block(block) => {
            for stmt in &block.stmts {
                recurse_and_find_sql(sqls, stmt, import_alias)?;
            }
        }
        Stmt::With(with_stmt) => {
            let stmt = *with_stmt.body.clone();
            recurse_and_find_sql(sqls, &stmt, import_alias)?;
        }
        Stmt::Return(rtn) => {
            if let Some(expr) = &rtn.arg {
                get_sql_from_expr(sqls, &expr.clone(), &import_alias);
            }
        }
        Stmt::If(if_stmt) => {
            let stmt = *if_stmt.cons.clone();
            recurse_and_find_sql(sqls, &stmt, import_alias)?;
        }
        Stmt::Switch(switch_stmt) => {
            for case in &switch_stmt.cases {
                for stmt in &case.cons {
                    recurse_and_find_sql(sqls, stmt, import_alias)?;
                }
            }
        }
        Stmt::Throw(throw_stmt) => {
            let expr = *throw_stmt.arg.clone();
            get_sql_from_expr(sqls, &expr, &import_alias);
        }
        Stmt::Try(try_stmt) => {
            for stmt in &try_stmt.block.stmts {
                recurse_and_find_sql(sqls, stmt, import_alias)?;
            }
            if let Some(stmt) = &try_stmt.handler {
                for stmt in &stmt.body.stmts {
                    recurse_and_find_sql(sqls, stmt, import_alias)?;
                }
            }
        }
        Stmt::While(while_stmt) => {
            let body_stmt = *while_stmt.body.clone();
            recurse_and_find_sql(sqls, &body_stmt, import_alias)?;
        }
        Stmt::DoWhile(do_while_stmt) => {
            let body_stmt = *do_while_stmt.body.clone();
            recurse_and_find_sql(sqls, &body_stmt, import_alias)?;
        }
        Stmt::For(for_stmt) => {
            let body_stmt = *for_stmt.body.clone();
            recurse_and_find_sql(sqls, &body_stmt, import_alias)?;
        }
        Stmt::ForIn(for_in_stmt) => {
            let body_stmt = *for_in_stmt.body.clone();
            recurse_and_find_sql(sqls, &body_stmt, import_alias)?;
        }
        Stmt::ForOf(for_of_stmt) => {
            let body_stmt = *for_of_stmt.body.clone();
            recurse_and_find_sql(sqls, &body_stmt, import_alias)?;
        }
        Stmt::Decl(decl) => {
            process_decl(sqls, decl, import_alias)?;
        }
        Stmt::Expr(expr) => {
            let expr = *expr.expr.clone();
            get_sql_from_expr(sqls, &expr, &import_alias);
        }
        Stmt::Empty(_) => {}
        Stmt::Debugger(_) => {}
        Stmt::Labeled(labeled) => {
            let body_stmt = *labeled.body.clone();
            recurse_and_find_sql(sqls, &body_stmt, import_alias)?;
        }
        Stmt::Break(_) => {}
        Stmt::Continue(_) => {}
    }
    Ok(())
}

pub fn process_block_stmt_as_expr(
    block_stmt: &Option<BlockStmt>,
    sqls: &mut Vec<String>,
    import_alias: &String,
) {
    if let Some(body) = block_stmt {
        for stmt in &body.stmts {
            let expr = stmt.as_expr();
            if let Some(expr) = expr {
                let expr = &expr.expr;
                get_sql_from_expr(sqls, expr, import_alias);
            } else {
                recurse_and_find_sql(sqls, stmt, import_alias).unwrap();
            }
        }
    }
}

pub fn get_var_decl_name(var_declarator: &VarDeclarator) -> Option<String> {
    match &var_declarator.name {
        Pat::Ident(ident) => Some(ident.id.sym.to_string()),
        Pat::Array(_) => None,
        Pat::Rest(_) => None,
        Pat::Object(_) => None,
        Pat::Assign(_) => None,
        Pat::Invalid(_) => None,
        Pat::Expr(_) => None,
    }
}

pub fn get_sql_from_expr(sqls: &mut Vec<String>, expr: &Expr, import_alias: &String) {
    match &expr {
        Expr::Call(call_expr) => {
            if let Some(callee_expr) = &call_expr.callee.as_expr() {
                if let Some(ident) = callee_expr.as_ident() {
                    if ident.sym.as_str() == import_alias && call_expr.args.len() == 1 {
                        let arg = call_expr.args.first().unwrap();

                        match &*arg.expr {
                            Expr::Lit(lit) => match lit {
                                Lit::Str(str) => {
                                    sqls.push(str.value.to_string());
                                }
                                _ => {}
                            },
                            Expr::Tpl(tpl) => {
                                for tpl_element in &tpl.quasis {
                                    sqls.push(tpl_element.raw.to_string());
                                }
                            }
                            Expr::TaggedTpl(tagged_tpl) => {
                                for tpl_element in &tagged_tpl.tpl.quasis {
                                    sqls.push(tpl_element.raw.to_string());
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            for arg in &call_expr.args {
                get_sql_from_expr(sqls, &arg.expr, import_alias);
            }
        }
        Expr::TsNonNull(expr) => {
            get_sql_from_expr(sqls, &expr.expr, import_alias);
        }
        Expr::TaggedTpl(_) => {}
        Expr::This(_) => {}
        Expr::Array(a) => {
            for elem in &a.elems {
                match elem {
                    Some(expr) => get_sql_from_expr(sqls, &expr.expr, import_alias),
                    None => {}
                }
            }
        }
        Expr::Object(object) => {
            for prop in &object.props {
                match prop {
                    PropOrSpread::Spread(_) => {}
                    PropOrSpread::Prop(prop) => match *prop.clone() {
                        Prop::Shorthand(_) => {}
                        Prop::KeyValue(key_val) => {
                            let value = &key_val.value;
                            get_sql_from_expr(sqls, value, import_alias);
                        }
                        Prop::Assign(assign) => {
                            let value = &assign.value;
                            get_sql_from_expr(sqls, value, import_alias);
                        }
                        Prop::Getter(getter) => {
                            let body = &getter.body;
                            process_block_stmt_as_expr(body, sqls, import_alias);
                        }
                        Prop::Setter(setter) => {
                            let body = &setter.body;
                            process_block_stmt_as_expr(body, sqls, import_alias);
                        }
                        Prop::Method(method) => {
                            let body = &method.function.body;
                            process_block_stmt_as_expr(body, sqls, import_alias);
                        }
                    },
                }
            }
        }
        Expr::Fn(_) => {}
        Expr::Unary(unary) => {
            let expr = &unary.arg;
            get_sql_from_expr(sqls, expr, import_alias)
        }
        Expr::Update(update) => {
            let expr = &update.arg;
            get_sql_from_expr(sqls, expr, import_alias)
        }
        Expr::Bin(bin) => {
            let left = &bin.left;
            let right = &bin.right;
            get_sql_from_expr(sqls, left, import_alias);
            get_sql_from_expr(sqls, right, import_alias);
        }
        Expr::Assign(assign) => {
            let right_expr = &assign.right;
            get_sql_from_expr(sqls, right_expr, import_alias);

            let left_expr = &assign.left;
            left_expr
                .as_expr()
                .map(|expr| get_sql_from_expr(sqls, expr, import_alias));
        }
        Expr::Member(member) => {
            let obj = &member.obj;
            get_sql_from_expr(sqls, obj, import_alias)
        }
        Expr::SuperProp(s) => {
            let super_prop = &s.prop;
            match &super_prop {
                SuperProp::Ident(_) => {}
                SuperProp::Computed(comp) => {
                    let expr = &comp.expr;
                    get_sql_from_expr(sqls, expr, import_alias)
                }
            }
        }
        Expr::Cond(cond) => {
            let test = &cond.test;
            let cons = &cond.cons;
            let alt = &cond.alt;
            get_sql_from_expr(sqls, test, import_alias);
            get_sql_from_expr(sqls, cons, import_alias);
            get_sql_from_expr(sqls, alt, import_alias);
        }
        Expr::New(expr) => {
            let args = &expr.args;
            let expr = &expr.callee;
            if let Some(args) = &args {
                for arg in args {
                    get_sql_from_expr(sqls, &arg.expr, import_alias);
                }
            }

            get_sql_from_expr(sqls, expr, import_alias);
        }
        Expr::Seq(seq) => {
            let exprs = &seq.exprs;
            for expr in exprs {
                get_sql_from_expr(sqls, expr, import_alias);
            }
        }
        Expr::Ident(_ident) => {}
        Expr::Lit(_lit) => {}
        Expr::Tpl(tpl) => {
            for expr in &tpl.exprs {
                get_sql_from_expr(sqls, expr, import_alias);
            }
        }
        Expr::Arrow(arrow) => {
            let expr = &arrow.clone().body.expr();
            let block_stmt = &arrow.clone().body.block_stmt();
            process_block_stmt_as_expr(block_stmt, sqls, import_alias);

            if let Some(expr) = expr {
                get_sql_from_expr(sqls, expr, import_alias);
            }

            for param in &arrow.params {
                let param = param.as_expr();
                if let Some(expr) = &param {
                    get_sql_from_expr(sqls, expr, import_alias);
                }
            }
        }
        Expr::Class(class) => {
            let class_body = &class.class.body;
            for body_stmt in class_body {
                match body_stmt {
                    ClassMember::Constructor(constructor) => {
                        if let Some(body) = &constructor.body {
                            for stmt in &body.stmts {
                                let expr = stmt.as_expr();
                                if let Some(expr) = expr {
                                    let expr = &expr.expr;
                                    return get_sql_from_expr(sqls, expr, import_alias);
                                }
                            }
                        }
                    }
                    ClassMember::Method(method) => {
                        let body = &method.function.body;
                        process_block_stmt_as_expr(body, sqls, import_alias);
                    }
                    ClassMember::PrivateMethod(private_method) => {
                        let body = &private_method.function.body;
                        process_block_stmt_as_expr(body, sqls, import_alias);
                    }
                    ClassMember::ClassProp(class_prop) => {
                        let body = &class_prop.value;
                        if let Some(body) = body {
                            return get_sql_from_expr(sqls, body, import_alias);
                        }
                    }
                    ClassMember::PrivateProp(private_prop) => {
                        let body = &private_prop.value;
                        if let Some(body) = body {
                            return get_sql_from_expr(sqls, body, import_alias);
                        }
                    }
                    ClassMember::TsIndexSignature(_) => {}
                    ClassMember::Empty(_) => {}
                    ClassMember::StaticBlock(static_block) => {
                        let body = &static_block.body;
                        process_block_stmt_as_expr(&Some(body.clone()), sqls, import_alias);
                    }
                    ClassMember::AutoAccessor(auto_accessor) => {
                        let value = &auto_accessor.value;

                        if let Some(expr) = &value {
                            get_sql_from_expr(sqls, expr, import_alias);
                        }
                    }
                }
            }
        }
        Expr::Yield(yield_expr) => {
            let expr = &yield_expr.arg;
            if let Some(expr) = expr {
                get_sql_from_expr(sqls, expr, import_alias)
            }
        }
        Expr::MetaProp(_) => {}
        Expr::Await(await_expr) => {
            let expr = &await_expr.arg;
            get_sql_from_expr(sqls, expr, import_alias)
        }
        Expr::Paren(paren) => {
            let expr = &paren.expr;
            get_sql_from_expr(sqls, expr, import_alias)
        }
        Expr::OptChain(opt_chain) => {
            let expr = &*opt_chain.base;
            match &expr {
                OptChainBase::Member(member) => {
                    let obj = &member.obj;
                    get_sql_from_expr(sqls, obj, import_alias);
                }
                OptChainBase::Call(call) => {
                    let expr = &call.callee;
                    get_sql_from_expr(sqls, expr, import_alias);

                    let args = &call.args;
                    for arg in args.iter() {
                        let expr = &arg.expr;
                        get_sql_from_expr(sqls, expr, import_alias);
                    }
                }
            }
        }
        Expr::JSXMember(_) => {}
        Expr::JSXNamespacedName(_) => {}
        Expr::JSXEmpty(_) => {}
        Expr::JSXElement(_) => {}
        Expr::JSXFragment(_) => {}
        Expr::TsTypeAssertion(_) => {}
        Expr::TsConstAssertion(_) => {}
        Expr::TsAs(_) => {}
        Expr::TsInstantiation(_) => {}
        Expr::PrivateName(_) => {}
        Expr::Invalid(_) => {}
        Expr::TsSatisfies(_) => {}
    }
}

pub fn get_sql_from_var_decl(var_declarator: &VarDeclarator, import_alias: &String) -> Vec<String> {
    let mut bag_of_sqls: Vec<String> = vec![];
    let var_decl_name = get_var_decl_name(var_declarator);

    if var_decl_name.is_none() {
        return bag_of_sqls;
    }

    if let Some(init) = &var_declarator.init {
        get_sql_from_expr(&mut bag_of_sqls, &&init.clone(), import_alias);
    }

    bag_of_sqls
}

fn process_class_member(
    sqls: &mut Vec<String>,
    body_stmt: &ClassMember,
    import_alias: &String,
) -> Result<()> {
    match body_stmt {
        ClassMember::Constructor(constructor) => {
            if let Some(body) = &constructor.body {
                for stmt in &body.stmts {
                    recurse_and_find_sql(sqls, stmt, import_alias)?;
                }
            }
        }
        ClassMember::Method(class_method) => {
            if let Some(body) = &class_method.function.body {
                for stmt in &body.stmts {
                    recurse_and_find_sql(sqls, stmt, import_alias)?;
                }
            }
        }
        ClassMember::PrivateMethod(private_method) => {
            if let Some(body) = &private_method.function.body {
                for stmt in &body.stmts {
                    recurse_and_find_sql(sqls, stmt, import_alias)?;
                }
            }
        }
        ClassMember::StaticBlock(static_block) => {
            for stmt in &static_block.body.stmts {
                recurse_and_find_sql(sqls, stmt, import_alias)?;
            }
        }
        ClassMember::PrivateProp(private_prop) => {
            if let Some(expr) = &private_prop.value {
                get_sql_from_expr(sqls, &expr.clone(), &import_alias);
            }
        }
        ClassMember::ClassProp(class_prop) => {
            if let Some(expr) = &class_prop.value {
                get_sql_from_expr(sqls, &expr.clone(), &import_alias);
            }
        }
        ClassMember::AutoAccessor(auto_accessor) => {
            let value = &auto_accessor.value;

            if let Some(expr) = &value {
                get_sql_from_expr(sqls, &expr, &import_alias);
            }
        }
        ClassMember::TsIndexSignature(_) => {}
        ClassMember::Empty(_) => {}
    }
    Ok(())
}

pub fn process_default_decl(
    sqls: &mut Vec<String>,
    default_decl: &DefaultDecl,
    import_alias: &String,
) -> Result<()> {
    match default_decl {
        DefaultDecl::Class(class) => {
            let class_body = &class.class.body;
            for body_stmt in class_body {
                process_class_member(sqls, body_stmt, import_alias)?;
            }
        }
        DefaultDecl::Fn(func) => {
            let body = &func.function.body;

            if let Some(body) = body {
                for stmt in &body.stmts {
                    recurse_and_find_sql(sqls, stmt, import_alias)?;
                }
            }
        }
        DefaultDecl::TsInterfaceDecl(_) => {}
    }
    Ok(())
}

pub fn process_class_decl(
    sqls: &mut Vec<String>,
    class: &ClassDecl,
    import_alias: &String,
) -> Result<()> {
    let class_body = &class.class.body;
    let class_decorators = &class.class.decorators;

    for decorator in class_decorators {
        let expr = &decorator.expr;
        get_sql_from_expr(sqls, expr, &import_alias);
    }

    for body_stmt in class_body {
        process_class_member(sqls, body_stmt, import_alias)?;
    }
    Ok(())
}

pub fn process_decl(sqls: &mut Vec<String>, decl: &Decl, import_alias: &String) -> Result<()> {
    match decl {
        Decl::Class(class) => {
            process_class_decl(sqls, class, import_alias)?;
        }
        Decl::Fn(fun) => {
            if let Some(body) = &fun.function.body {
                for stmt in &body.stmts {
                    recurse_and_find_sql(sqls, stmt, import_alias)?;
                }
            }
        }
        Decl::Var(var) => {
            for var_decl in &var.decls {
                let new_sqls = get_sql_from_var_decl(var_decl, &import_alias);
                let num_new_sqls = new_sqls.len();

                sqls.extend(new_sqls);

                if num_new_sqls > 0 {
                    continue;
                }
                if let Some(init) = &var_decl.init {
                    let expr = *init.clone();
                    get_sql_from_expr(sqls, &expr, &import_alias);
                }
            }
        }
        Decl::TsInterface(_) => {}
        Decl::TsTypeAlias(_) => {}
        Decl::TsEnum(_) => {}
        Decl::TsModule(module) => {
            for stmt in &module.body {
                for block in &stmt.as_ts_module_block() {
                    for body in &block.body {
                        let stmt = &body.clone().stmt();
                        if let Some(stmt) = stmt {
                            recurse_and_find_sql(sqls, stmt, import_alias)?;
                        }
                    }
                }
            }
        }
        Decl::Using(using) => {
            for decl in &using.decls {
                let init = &decl.init;
                if let Some(expr) = init {
                    get_sql_from_expr(sqls, expr, import_alias);
                }
            }
        }
    }
    Ok(())
}
