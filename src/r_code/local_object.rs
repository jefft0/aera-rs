use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Write;
#[cfg(with_detail_oid)]
use std::sync::atomic::{AtomicU64, Ordering};
use super::atom;
use super::atom::Atom;
use super::code::Code;
use super::code::CodeTrace;


#[cfg(with_detail_oid)]
// Start with a non-zero value so that it doesn't appear to track object OIDs.
static LAST_DETAIL_OID: AtomicU64 = AtomicU64::new(11);

pub struct LocalObject {
    oid_: u32,
    code_: Vec<Atom>,
    references_: Vec<Rc<RefCell<dyn Code>>>,
    #[cfg(with_detail_oid)]
    detail_oid_: u64,
}

impl Default for LocalObject {
    fn default() -> Self {
        LocalObject { oid_: 0, code_: Vec::default(), references_: Vec::default(),
            #[cfg(with_detail_oid)]
            detail_oid_: LAST_DETAIL_OID.fetch_add(1, Ordering::SeqCst),
        }
    }
}

impl Code for LocalObject {
    #[cfg(with_detail_oid)]
    fn get_detail_oid(&self) -> u64 {
        self.detail_oid_
    }

    #[cfg(with_detail_oid)]
    fn set_detail_oid(&mut self, detail_oid: u64) {
        self.detail_oid_ = detail_oid;
        // Make sure the next assigned detail OID is higher.
        LAST_DETAIL_OID.store(detail_oid + 1, Ordering::Relaxed);
    }

    fn get_oid(&self) -> u32 {
        self.oid_
    }

    fn set_oid(&mut self, oid: u32) {
        self.oid_ = oid;
    }

    fn code(&self, i: u16) -> Atom {
        self.code_[i as usize]
    }

    fn set_code(&mut self, i: u16, a: Atom) {
        if i >= self.code_size() {
            self.resize_code(i + 1);
        }
        self.code_[i as usize] = a;
    }

    fn code_size(&self) -> u16 {
        self.code_.len() as u16
    }

    fn resize_code(&mut self, new_size: u16) {
        self.code_.resize(new_size as usize, Atom::default());
    }

    fn set_reference(&mut self, i: u16, object: &Rc<RefCell<dyn Code>>) {
        self.references_[i as usize] = Rc::clone(&object);
    }

    fn get_reference(&self, i: u16) -> Rc<RefCell<dyn Code>> {
        Rc::clone(&self.references_[i as usize])
    }

    fn references_size(&self) -> u16 {
        self.references_.len() as u16
    }

    fn clear_references(&mut self) {
        self.references_.clear();
    }
}

impl CodeTrace for LocalObject {
    fn trace_at(&self, i: u16, out: &mut impl Write, context: &mut atom::TraceContext) {
        super::code::trace_at(self, i, out, context);
    }

    fn trace_out(&self, out: &mut impl Write) {
        super::code::trace_out(self, out);
    }
}
