use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Write;
use super::atom;
use super::atom::Atom;

//const NULL_STORAGE_INDEX: isize = -1;
//const CODE_MARKERS_INITIAL_SIZE: usize = 8;

pub trait Code {
    #[cfg(with_detail_oid)]
    // Compile with: RUSTFLAGS='--cfg with_detail_oid' cargo build
    fn get_detail_oid(&self) -> u64;

    #[cfg(with_detail_oid)]
    /**
     * Set this object's detail OID and also set the static last_detail_oid
     * so that the next detail OID will be higher than this one.
     * \param detail_oid The detail OID.
     */
     fn set_detail_oid(&mut self, detail_oid: u64);
   
/* TODO: Implement
    int32 storage_index_; // -1: not stored; >= 0: index of the object in a vector-based container.

    void load(SysObject *source) {
  
      for (uint16 i = 0; i < source->code_.size(); ++i)
        code(i) = source->code_[i];
      set_oid(source->oid_);
    }
    template<class V> _View *build_view(SysView *source) {
  
      return new V(source, this);
    }

    void set_strorage_index(int32 i) { storage_index_ = i; }
    bool is_registered() const { return storage_index_ > null_storage_index; }
    int32 get_storage_index() const { return storage_index_; }
*/

    fn get_oid(&self) -> u32;
    fn set_oid(&mut self, oid: u32);

    fn code(&self, i: u16) -> Atom;
    fn set_code(&mut self, i: u16, a: Atom);
    fn code_size(&self) -> u16;
    fn resize_code(&mut self, new_size: u16);
    fn set_reference(&mut self, i: u16, object: &Rc<RefCell<dyn Code>>);
    fn get_reference(&self, i: u16) -> Rc<RefCell<dyn Code>>;
    fn references_size(&self) -> u16;
    fn clear_references(&mut self);
/* TODO: Implement
  virtual void set_references(std::vector<P<Code> > &new_references) = 0;

  virtual bool is_compact() const { return false; }
  virtual bool is_invalidated() { return false; }
  virtual bool invalidate() { return false; }

  r_code::list<Code *> markers_;
  std::unordered_set<_View *, _View::Hash, _View::Equal> views_; // indexed by groups.

  virtual _View *build_view(SysView *source) = 0;

  virtual void acq_views() {}
  virtual void rel_views() {}
  virtual void acq_markers() {}
  virtual void rel_markers() {}

  virtual float32 get_psln_thr() { return 1; }

  Code() : storage_index_(null_storage_index) { markers_.reserve(CodeMarkersInitialSize); }
  virtual ~Code() {}

  virtual void mod(uint16 member_index, float32 value) {};
  virtual void set(uint16 member_index, float32 value) {};
  virtual _View *get_view(Code *group, bool lock) { return NULL; }
  virtual void add_reference(Code *object) const {} // called only on local objects.
  void remove_marker(Code *m) {

    acq_markers();
    markers_.remove(m);
    rel_markers();
  }
*/
}

pub trait CodeTrace {
    /**
     * Print the trace of code(i) to the out stream, using the given TraceContext.
     */
    fn trace_at(&self, i: u16, out: &mut impl Write, context: &mut atom::TraceContext);

    /**
     * Print the trace of this Code to the out stream.
     */
    fn trace_out(&self, out: &mut impl Write);

    /**
     * Print the trace of this Code to stdout.
     */
    fn trace(&self) {
        // Debug: Should use trace_out(io::stdout()). But need std::fmt::Write or std::io::Write.
        print!("{}", self.trace_string());
    }

    /*
     * Return the trace as a string. For debugging purposes only(can be inefficient).
     */
     fn trace_string(&self) -> String {
        let mut out = String::new();
        self.trace_out(&mut out);
        out
    }
}

pub fn trace_at(code: &impl Code, i: u16, out: &mut impl Write, context: &mut atom::TraceContext) {
    let a = code.code(i);
    a.trace(context, out);
    if a.getDescriptor() == atom::R_PTR {
        if a.asIndex() < code.references_size() {
            write!(out, " -> {}", code.get_reference(a.asIndex()).borrow().get_oid()).unwrap();
            #[cfg(with_detail_oid)]
            write!(out, "({})", code.get_reference(a.asIndex()).borrow().get_detail_oid()).unwrap();
        }
        else {
            write!(out, " (unassigned) ").unwrap();
        }
    }
}

pub fn trace_out(code: &impl Code, out: &mut impl Write) {
    write!(out, "--------\n").unwrap();
    let mut context = atom::TraceContext::default();
    for i in 0..code.code_size() {
        write!(out, "{}\t", i).unwrap();
        trace_at(code, i, out, &mut context);
        write!(out, "\n").unwrap();
    }
    write!(out, "OID: {}", code.get_oid()).unwrap();
    #[cfg(with_detail_oid)]
    write!(out, "({})", code.get_detail_oid()).unwrap();
    write!(out, "\n").unwrap();
}
