use std::rc::Rc;
use std::fmt;
use ::types::{Type,TyVar,Kind};

#[derive(Debug)]
pub struct Module {
    name:  String,
    decls: Vec<Decl>,
}

#[derive(Debug)]
pub enum Decl {
    Extern(TermVar),
    Let(TermVar, Expr),
}

#[derive(Clone,Hash,Eq,PartialEq)]
pub struct TermVar {
    name: Rc<String>,
    id: u32,
    ty: Type
}

#[derive(Clone)]
pub enum Var {
    Term(TermVar),
    Type(TyVar, Kind)
}

pub struct Let {
    id:   TermVar,
    bind: Expr,
    expr: Expr,
}

#[derive(Debug)]
pub struct If {
    cond:  Expr,
    texpr: Expr,
    fexpr: Expr,
    ty: Type
}

#[derive(Debug)]
pub enum Expr {
    UnitLit,
    I32Lit(i32),
    BoolLit(bool),
    Var(TermVar),
    If(Box<If>),
    Let(Box<Let>),
    Lam(Vec<TermVar>, Box<Expr>),
    App(u32, Box<Expr>, Box<Expr>),
}

impl Module {
    pub fn new(name: String, decls: Vec<Decl>) -> Self {
        Self{name, decls}
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn decls(&self) -> &Vec<Decl> {
        &self.decls
    }

    pub fn take_decls(self) -> Vec<Decl> {
        self.decls
    }

    pub fn add_decl(&mut self, decl: Decl) {
        self.decls.push(decl)
    }
}

impl TermVar {
    pub fn new(name: Rc<String>, ty: Type, id: u32) -> Self {
        TermVar{name, ty, id}
    }
    pub fn with_ty(&self, ty: Type) -> TermVar {
        TermVar::new(self.name.clone(), ty, self.id)
    }
    pub fn name(&self) -> &Rc<String> {
        &self.name
    }
    pub fn ty(&self) -> &Type {
        &self.ty
    }
    pub fn id(&self) -> u32 {
        self.id
    }
}

impl fmt::Debug for TermVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}_{}: {:?}", self.name, self.id, self.ty)
    }
}

impl If {
    pub fn new(cond: Expr, texpr: Expr, fexpr: Expr, ty: Type) -> Self {
        If{cond, texpr, fexpr, ty}
    }
    pub fn cond(&self) -> &Expr {
        &self.cond
    }
    pub fn texpr(&self) -> &Expr {
        &self.texpr
    }
    pub fn fexpr(&self) -> &Expr {
        &self.fexpr
    }
    pub fn ty(&self) -> &Type {
        &self.ty
    }
}

impl Let {
    pub fn new(id: TermVar, bind: Expr, expr: Expr) -> Self {
        Let{id, bind: bind, expr}
    }
    pub fn id(&self) -> &TermVar {
        &self.id
    }
    pub fn bind(&self) -> &Expr {
        &self.bind
    }
    pub fn expr(&self) -> &Expr {
        &self.expr
    }
}

impl fmt::Debug for Let {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "let {:?} = {:?}\n{:?}", self.id, self.bind, self.expr)
    }
}
