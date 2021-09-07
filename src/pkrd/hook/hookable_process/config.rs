/// Configuration used to patch a game's `gspPresentBuffer` function.
pub struct PatchPresentFramebufferConfig {
    /// Determines if a game uses extended memory mode.
    /// usum and sm will be true.  All other games should be false.
    pub is_extended_memory: bool,
    /// `gspPresentBuffer` in libctru.
    /// This can be found by searching for the byte sequence: `9a 8f 07 ee`, which is `mcr p15,0x0,r8,cr7,cr10,0x4`.
    /// You may need to modify the search to account for registers other than r8, but r8 is usually used.
    pub present_framebuffer_addr: u32,
    /// The address of the first function called in `gspPresentBuffer`.
    pub get_screen_addr: u32,
    /// Most games will probably require this to be aligned with 0x10000,
    /// but that's not necessarilly true for all games.
    pub hook_vars_addr: u32,
}

impl PatchPresentFramebufferConfig {
    pub fn get_heap_addr(&self) -> u32 {
        if self.is_extended_memory {
            0x30000000
        } else {
            0x8000000
        }
    }

    pub fn get_heap_size(&self) -> u32 {
        if self.is_extended_memory {
            0x34a8d84
        } else {
            0x6000000
        }
    }
}
