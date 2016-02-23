use super::super::ast::*;

pub trait VisitorMut<'v> : Sized {
    fn visit_mut_module(&mut self, module: &'v mut Module) {
        walk_mut_module(self, module)
    }

    fn visit_mut_item(&mut self, item: &'v mut Item) {
        walk_mut_item(self, item)
    }

    fn visit_mut_data(&mut self, inductive: &'v mut Inductive) {
        walk_mut_inductive(self, inductive)
    }

    fn visit_mut_extern(&mut self, ext: &'v mut Extern) {
        panic!();
    }

    fn visit_mut_def(&mut self, def: &'v mut Def) {
        walk_mut_def(self, def)
    }

    fn visit_mut_term(&mut self, term: &'v mut Term) {
        walk_mut_term(self, term)
    }

    fn visit_mut_binder(&mut self, binder: &'v mut Binder) {
        walk_mut_binder(self, binder)
    }

    fn visit_mut_span(&mut self, _span: &'v mut Span) {}

    fn visit_mut_case(&mut self, case: &'v mut Case) {
        panic!();
    }

    fn visit_mut_pattern(&mut self, pattern: &'v mut Pattern) {
        panic!();
    }

    fn visit_mut_literal(&mut self, lit: &'v mut Literal) {
        panic!();
    }

    fn visit_mut_name(&mut self, name: &'v mut Name) {
        walk_mut_name(self, name)
    }
}

pub fn walk_mut_module<'v, V: VisitorMut<'v>>(visitor: &mut V, module: &'v mut Module) {
    visitor.visit_mut_span(&mut module.span);
    visitor.visit_mut_name(&mut module.name);

    for item in &mut module.items {
        visitor.visit_mut_item(item);
    }
}

pub fn walk_mut_item<'v, V: VisitorMut<'v>>(visitor: &mut V, item: &'v mut Item) {
    use ast::Item::*;

    match item {
        &mut Item::Inductive(ref mut d) => visitor.visit_mut_data(d),
        &mut Item::Def(ref mut def) => visitor.visit_mut_def(def),
        &mut Item::Extern(ref mut ext) => panic!(),
        &mut Item::Comment(ref mut s) => panic!(),
        &mut Item::Import(ref mut n) => visitor.visit_mut_name(n),
    }
}

pub fn walk_mut_inductive<'v, V: VisitorMut<'v>>(visitor: &mut V, inductive: &'v mut Inductive) {
    visitor.visit_mut_span(&mut inductive.span);
    visitor.visit_mut_name(&mut inductive.name);

    for binder in &mut inductive.parameters {
        visitor.visit_mut_binder(binder);
    }

    visitor.visit_mut_term(&mut inductive.ty);

    for &mut (ref mut n, ref mut t) in &mut inductive.ctors {
        visitor.visit_mut_name(n);
        visitor.visit_mut_term(t);
    }
}

pub fn walk_mut_def<'v, V: VisitorMut<'v>>(visitor: &mut V, def: &'v mut Def) {
    visitor.visit_mut_span(&mut def.span);
    visitor.visit_mut_name(&mut def.name);

    for binder in &mut def.args {
        visitor.visit_mut_binder(binder);
    }

    visitor.visit_mut_term(&mut def.ty);
    visitor.visit_mut_term(&mut def.body);
}

pub fn walk_mut_term<'v, V: VisitorMut<'v>>(visitor: &mut V, term: &'v mut Term) {
    use ast::Term::*;

    match term {
        &mut Literal { ref mut span, ref mut lit } =>
            panic!(),
        &mut Var { ref mut name } =>
            visitor.visit_mut_name(name),
        &mut Match { ref mut span, ref mut scrutinee, ref mut cases } =>
            panic!(),
        &mut App { ref mut span, ref mut fun, ref mut arg } => {
            visitor.visit_mut_span(span);
            visitor.visit_mut_term(fun);
            visitor.visit_mut_term(arg);
        }
        &mut Forall { ref mut span, ref mut binders, ref mut term } => {
            visitor.visit_mut_span(span);
            for binder in binders {
                visitor.visit_mut_binder(binder);
            }

            visitor.visit_mut_term(term);
        }
        &mut Lambda { ref mut span, ref mut args, ref mut ret_ty, ref mut body } => {
            visitor.visit_mut_span(span);
            for binder in args {
                visitor.visit_mut_binder(binder);
            }

            match **ret_ty {
                None => {}
                Some(ref mut rt) =>
                    visitor.visit_mut_term(rt),
            }

            visitor.visit_mut_term(body);
        }
        &mut Let { ref mut span, ref mut bindings, ref mut body } => {
            visitor.visit_mut_span(span);
            panic!()
        }
        &mut Type => {}
    }
}

pub fn walk_mut_name<'v, V: VisitorMut<'v>>(visitor: &mut V, name: &'v mut Name) {
    visitor.visit_mut_span(&mut name.span);
}

pub fn walk_mut_binder<'v, V: VisitorMut<'v>>(visitor: &mut V, binder: &'v mut Binder) {
    visitor.visit_mut_span(&mut binder.span);
    for name in &mut binder.names {
        visitor.visit_mut_name(name);
    }
    visitor.visit_mut_term(&mut binder.ty);
}
