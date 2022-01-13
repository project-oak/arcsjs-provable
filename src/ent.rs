use std::borrow::Borrow;
use super::context::{Ctx, CTX};

pub type EntityIdBackingType = u64;

#[derive(Copy, Clone, PartialOrd, Ord, Eq, PartialEq, Hash)]
pub struct Ent {
    pub id: EntityIdBackingType,
}

impl Ent {
    fn new(ctx: &mut Ctx, name: &str) -> Self {
        let id = ctx.last_id;
        ctx.last_id += 1;
        let ent = Ent { id };
        ctx.name_to_id.insert(name.to_string(), ent);
        ctx.id_to_name.insert(ent, name.to_string());
        ent
    }

    pub fn name(&self) -> String {
        let guard = CTX.lock().expect("Shouldn't fail");
        let ctx = (*guard).borrow();
        ctx.borrow()
            .id_to_name
            .get(self)
            .cloned()
            .expect("All entities should have a name")
    }

    fn get_by_name(ctx: &mut Ctx, name: &str) -> Option<Ent> {
        ctx.name_to_id.get(name).cloned()
    }

    pub fn by_name(name: &str) -> Ent {
        let guard = CTX.lock().expect("Shouldn't fail");
        let mut ctx = (*guard).borrow_mut();
        Ent::get_by_name(&mut ctx, name).unwrap_or_else(|| Ent::new(&mut ctx, name))
    }
}

impl std::fmt::Display for Ent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::fmt::Debug for Ent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ent")
            .field("id", &self.id)
            .field("{name}", &self.name())
            .finish()
    }
}
