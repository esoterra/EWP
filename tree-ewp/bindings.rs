pub mod parser1 {
  #[allow(unused_imports)]
  use wit_bindgen_wasmtime::{wasmtime, anyhow};
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
  impl wit_bindgen_wasmtime::Endian for Span {
    fn into_le(self) -> Self {
      Self {
        offset: self.offset.into_le(),
        length: self.length.into_le(),
      }
    }
    fn from_le(self) -> Self {
      Self {
        offset: self.offset.from_le(),
        length: self.length.from_le(),
      }
    }
  }
  unsafe impl wit_bindgen_wasmtime::AllBytesValid for Span {}
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
  
  /// Auxiliary data associated with the wasm exports.
  ///
  /// This is required to be stored within the data of a
  /// `Store<T>` itself so lifting/lowering state can be managed
  /// when translating between the host and wasm.
  #[derive(Default)]
  pub struct Parser1Data {
  }
  pub struct Parser1<T> {
    get_state: Box<dyn Fn(&mut T) -> &mut Parser1Data + Send + Sync>,
    canonical_abi_free: wasmtime::TypedFunc<(i32, i32, i32), ()>,
    canonical_abi_realloc: wasmtime::TypedFunc<(i32, i32, i32, i32), i32>,
    memory: wasmtime::Memory,
    parse: wasmtime::TypedFunc<(i32,i32,), (i32,)>,
  }
  impl<T> Parser1<T> {
    #[allow(unused_variables)]
    
    /// Adds any intrinsics, if necessary for this exported wasm
    /// functionality to the `linker` provided.
    ///
    /// The `get_state` closure is required to access the
    /// auxiliary data necessary for these wasm exports from
    /// the general store's state.
    pub fn add_to_linker(
    linker: &mut wasmtime::Linker<T>,
    get_state: impl Fn(&mut T) -> &mut Parser1Data + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<()> {
      Ok(())
    }
    
    /// Instantiates the provided `module` using the specified
    /// parameters, wrapping up the result in a structure that
    /// translates between wasm and the host.
    ///
    /// The `linker` provided will have intrinsics added to it
    /// automatically, so it's not necessary to call
    /// `add_to_linker` beforehand. This function will
    /// instantiate the `module` otherwise using `linker`, and
    /// both an instance of this structure and the underlying
    /// `wasmtime::Instance` will be returned.
    ///
    /// The `get_state` parameter is used to access the
    /// auxiliary state necessary for these wasm exports from
    /// the general store state `T`.
    pub fn instantiate(
    mut store: impl wasmtime::AsContextMut<Data = T>,
    module: &wasmtime::Module,
    linker: &mut wasmtime::Linker<T>,
    get_state: impl Fn(&mut T) -> &mut Parser1Data + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<(Self, wasmtime::Instance)> {
      Self::add_to_linker(linker, get_state)?;
      let instance = linker.instantiate(&mut store, module)?;
      Ok((Self::new(store, &instance,get_state)?, instance))
    }
    
    /// Low-level creation wrapper for wrapping up the exports
    /// of the `instance` provided in this structure of wasm
    /// exports.
    ///
    /// This function will extract exports from the `instance`
    /// defined within `store` and wrap them all up in the
    /// returned structure which can be used to interact with
    /// the wasm module.
    pub fn new(
    mut store: impl wasmtime::AsContextMut<Data = T>,
    instance: &wasmtime::Instance,
    get_state: impl Fn(&mut T) -> &mut Parser1Data + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<Self> {
      let mut store = store.as_context_mut();
      let canonical_abi_free= instance.get_typed_func::<(i32, i32, i32), (), _>(&mut store, "canonical_abi_free")?;
      let canonical_abi_realloc= instance.get_typed_func::<(i32, i32, i32, i32), i32, _>(&mut store, "canonical_abi_realloc")?;
      let memory= instance
      .get_memory(&mut store, "memory")
      .ok_or_else(|| {
        anyhow::anyhow!("`memory` export not a memory")
      })?
      ;
      let parse= instance.get_typed_func::<(i32,i32,), (i32,), _>(&mut store, "parse")?;
      Ok(Parser1{
        canonical_abi_free,
        canonical_abi_realloc,
        memory,
        parse,
        get_state: Box::new(get_state),
        
      })
    }
    pub fn parse(&self, mut caller: impl wasmtime::AsContextMut<Data = T>,input: & str,)-> Result<Output, wasmtime::Trap> {
      let func_canonical_abi_realloc = &self.canonical_abi_realloc;
      let func_canonical_abi_free = &self.canonical_abi_free;
      let memory = &self.memory;
      let vec0 = input;
      let ptr0 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec0.len() as i32) * 1))?;
      memory.data_mut(&mut caller).store_many(ptr0, vec0.as_ref())?;
      let (result1_0,) = self.parse.call(&mut caller, (ptr0, vec0.len() as i32, ))?;
      let load2 = memory.data_mut(&mut caller).load::<i32>(result1_0 + 0)?;
      let load3 = memory.data_mut(&mut caller).load::<i32>(result1_0 + 8)?;
      let load4 = memory.data_mut(&mut caller).load::<i32>(result1_0 + 16)?;
      let load5 = memory.data_mut(&mut caller).load::<i32>(result1_0 + 24)?;
      let len11 = load3;
      let base11 = load2;
      let mut result11 = Vec::with_capacity(len11 as usize);
      for i in 0..len11 {
        let base = base11 + i *16;
        result11.push({
          let load6 = memory.data_mut(&mut caller).load::<i32>(base + 0)?;
          let load7 = memory.data_mut(&mut caller).load::<i32>(base + 4)?;
          let ptr8 = load6;
          let len8 = load7;
          
          let data8 = copy_slice(
          &mut caller,
          memory,
          ptr8, len8, 1
          )?;
          func_canonical_abi_free.call(&mut caller, (ptr8, len8 * 1, 1))?;
          let load9 = memory.data_mut(&mut caller).load::<i32>(base + 8)?;
          let load10 = memory.data_mut(&mut caller).load::<i32>(base + 12)?;
          Token{label:String::from_utf8(data8)
          .map_err(|_| wasmtime::Trap::new("invalid utf-8"))?, span:Span{offset:load9 as u32, length:load10 as u32, }, }
        });
      }
      func_canonical_abi_free.call(&mut caller, (base11, len11 * 16, 4))?;
      let len21 = load5;
      let base21 = load4;
      let mut result21 = Vec::with_capacity(len21 as usize);
      for i in 0..len21 {
        let base = base21 + i *16;
        result21.push({
          let load12 = memory.data_mut(&mut caller).load::<i32>(base + 0)?;
          let load13 = memory.data_mut(&mut caller).load::<i32>(base + 4)?;
          let ptr14 = load12;
          let len14 = load13;
          
          let data14 = copy_slice(
          &mut caller,
          memory,
          ptr14, len14, 1
          )?;
          func_canonical_abi_free.call(&mut caller, (ptr14, len14 * 1, 1))?;
          let load15 = memory.data_mut(&mut caller).load::<i32>(base + 8)?;
          let load16 = memory.data_mut(&mut caller).load::<i32>(base + 12)?;
          let len20 = load16;
          let base20 = load15;
          let mut result20 = Vec::with_capacity(len20 as usize);
          for i in 0..len20 {
            let base = base20 + i *8;
            result20.push({
              let load17 = memory.data_mut(&mut caller).load::<u8>(base + 0)?;
              match i32::from(load17) {
                0 => NodeIndex::Token({
                  let load18 = memory.data_mut(&mut caller).load::<i32>(base + 4)?;
                  load18 as u32
                }),
                1 => NodeIndex::Branch({
                  let load19 = memory.data_mut(&mut caller).load::<i32>(base + 4)?;
                  load19 as u32
                }),
                _ => return Err(invalid_variant("NodeIndex")),
              }
            });
          }
          func_canonical_abi_free.call(&mut caller, (base20, len20 * 8, 4))?;
          Branch{label:String::from_utf8(data14)
          .map_err(|_| wasmtime::Trap::new("invalid utf-8"))?, children:result20, }
        });
      }
      func_canonical_abi_free.call(&mut caller, (base21, len21 * 16, 4))?;
      Ok(Output{tokens:result11, tree:result21, })
    }
  }
  use wit_bindgen_wasmtime::rt::RawMem;
  use wit_bindgen_wasmtime::rt::invalid_variant;
  use wit_bindgen_wasmtime::rt::copy_slice;
}
