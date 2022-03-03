mod ewp0 {
  #[derive(Clone)]
  pub struct Token {
    pub label: String,
    pub span: Span,
  }
  impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("Token").field("label", &self.label).field("span", &self.span).finish()}
  }
  #[repr(C)]
  #[derive(Copy, Clone)]
  pub struct Span {
    pub offset: u32,
    pub length: u32,
  }
  impl std::fmt::Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("Span").field("offset", &self.offset).field("length", &self.length).finish()}
  }
  #[derive(Clone)]
  pub struct Branch {
    pub label: String,
    pub children: Vec<NodeIndex>,
  }
  impl std::fmt::Debug for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("Branch").field("label", &self.label).field("children", &self.children).finish()}
  }
  #[derive(Clone, Copy)]
  pub enum NodeIndex{
    Token(u32),
    Branch(u32),
  }
  impl std::fmt::Debug for NodeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        NodeIndex::Token(e) => {
          f.debug_tuple("NodeIndex::Token").field(e).finish()
        }
        NodeIndex::Branch(e) => {
          f.debug_tuple("NodeIndex::Branch").field(e).finish()
        }
      }
    }
  }
  #[derive(Clone)]
  pub struct Output {
    pub tokens: Vec<Token>,
    pub tree: Vec<Branch>,
  }
  impl std::fmt::Debug for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("Output").field("tokens", &self.tokens).field("tree", &self.tree).finish()}
  }
  #[export_name = "parse"]
  unsafe extern "C" fn __wit_bindgen_parse(arg0: i32, arg1: i32, ) -> i32{
    let len0 = arg1 as usize;
    let result1 = <super::Ewp0 as Ewp0>::parse(String::from_utf8(Vec::from_raw_parts(arg0 as *mut _, len0, len0)).unwrap());
    let Output{ tokens:tokens2, tree:tree2, } = result1;
    let vec6 = tokens2;
    let len6 = vec6.len() as i32;
    let layout6 = core::alloc::Layout::from_size_align_unchecked(vec6.len() * 16, 4);
    let result6 = std::alloc::alloc(layout6);
    if result6.is_null() { std::alloc::handle_alloc_error(layout6); }
    for (i, e) in vec6.into_iter().enumerate() {
      let base = result6 as i32 + (i as i32) * 16;
      {
        let Token{ label:label3, span:span3, } = e;
        let vec4 = (label3.into_bytes()).into_boxed_slice();
        let ptr4 = vec4.as_ptr() as i32;
        let len4 = vec4.len() as i32;
        core::mem::forget(vec4);
        *((base + 4) as *mut i32) = len4;
        *((base + 0) as *mut i32) = ptr4;
        let Span{ offset:offset5, length:length5, } = span3;
        *((base + 8) as *mut i32) = wit_bindgen_rust::rt::as_i32(offset5);
        *((base + 12) as *mut i32) = wit_bindgen_rust::rt::as_i32(length5);
        
      }}
      let vec10 = tree2;
      let len10 = vec10.len() as i32;
      let layout10 = core::alloc::Layout::from_size_align_unchecked(vec10.len() * 16, 4);
      let result10 = std::alloc::alloc(layout10);
      if result10.is_null() { std::alloc::handle_alloc_error(layout10); }
      for (i, e) in vec10.into_iter().enumerate() {
        let base = result10 as i32 + (i as i32) * 16;
        {
          let Branch{ label:label7, children:children7, } = e;
          let vec8 = (label7.into_bytes()).into_boxed_slice();
          let ptr8 = vec8.as_ptr() as i32;
          let len8 = vec8.len() as i32;
          core::mem::forget(vec8);
          *((base + 4) as *mut i32) = len8;
          *((base + 0) as *mut i32) = ptr8;
          let vec9 = children7;
          let len9 = vec9.len() as i32;
          let layout9 = core::alloc::Layout::from_size_align_unchecked(vec9.len() * 8, 4);
          let result9 = std::alloc::alloc(layout9);
          if result9.is_null() { std::alloc::handle_alloc_error(layout9); }
          for (i, e) in vec9.into_iter().enumerate() {
            let base = result9 as i32 + (i as i32) * 8;
            {
              match e{
                NodeIndex::Token(e) => { {
                  *((base + 0) as *mut u8) = (0i32) as u8;
                  *((base + 4) as *mut i32) = wit_bindgen_rust::rt::as_i32(e);
                  
                }}
                NodeIndex::Branch(e) => { {
                  *((base + 0) as *mut u8) = (1i32) as u8;
                  *((base + 4) as *mut i32) = wit_bindgen_rust::rt::as_i32(e);
                  
                }}
              };
              
            }}
            *((base + 12) as *mut i32) = len9;
            *((base + 8) as *mut i32) = result9 as i32;
            
          }}
          let ptr11 = RET_AREA.as_mut_ptr() as i32;
          *((ptr11 + 24) as *mut i32) = len10;
          *((ptr11 + 16) as *mut i32) = result10 as i32;
          *((ptr11 + 8) as *mut i32) = len6;
          *((ptr11 + 0) as *mut i32) = result6 as i32;
          ptr11
        }
        pub trait Ewp0 {
          fn parse(input: String,) -> Output;
        }
        static mut RET_AREA: [i64; 4] = [0; 4];
      }
      