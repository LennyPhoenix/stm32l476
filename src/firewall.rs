#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cssa: Cssa,
    csl: Csl,
    nvdssa: Nvdssa,
    nvdsl: Nvdsl,
    vdssa: Vdssa,
    vdsl: Vdsl,
    _reserved6: [u8; 0x08],
    cr: Cr,
}
impl RegisterBlock {
    #[doc = "0x00 - Code segment start address"]
    #[inline(always)]
    pub const fn cssa(&self) -> &Cssa {
        &self.cssa
    }
    #[doc = "0x04 - Code segment length"]
    #[inline(always)]
    pub const fn csl(&self) -> &Csl {
        &self.csl
    }
    #[doc = "0x08 - Non-volatile data segment start address"]
    #[inline(always)]
    pub const fn nvdssa(&self) -> &Nvdssa {
        &self.nvdssa
    }
    #[doc = "0x0c - Non-volatile data segment length"]
    #[inline(always)]
    pub const fn nvdsl(&self) -> &Nvdsl {
        &self.nvdsl
    }
    #[doc = "0x10 - Volatile data segment start address"]
    #[inline(always)]
    pub const fn vdssa(&self) -> &Vdssa {
        &self.vdssa
    }
    #[doc = "0x14 - Volatile data segment length"]
    #[inline(always)]
    pub const fn vdsl(&self) -> &Vdsl {
        &self.vdsl
    }
    #[doc = "0x20 - Configuration register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
}
#[doc = "CSSA (rw) register accessor: Code segment start address\n\nYou can [`read`](crate::Reg::read) this register and get [`cssa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cssa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cssa`] module"]
#[doc(alias = "CSSA")]
pub type Cssa = crate::Reg<cssa::CssaSpec>;
#[doc = "Code segment start address"]
pub mod cssa;
#[doc = "CSL (rw) register accessor: Code segment length\n\nYou can [`read`](crate::Reg::read) this register and get [`csl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csl`] module"]
#[doc(alias = "CSL")]
pub type Csl = crate::Reg<csl::CslSpec>;
#[doc = "Code segment length"]
pub mod csl;
#[doc = "NVDSSA (rw) register accessor: Non-volatile data segment start address\n\nYou can [`read`](crate::Reg::read) this register and get [`nvdssa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvdssa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvdssa`] module"]
#[doc(alias = "NVDSSA")]
pub type Nvdssa = crate::Reg<nvdssa::NvdssaSpec>;
#[doc = "Non-volatile data segment start address"]
pub mod nvdssa;
#[doc = "NVDSL (rw) register accessor: Non-volatile data segment length\n\nYou can [`read`](crate::Reg::read) this register and get [`nvdsl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvdsl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvdsl`] module"]
#[doc(alias = "NVDSL")]
pub type Nvdsl = crate::Reg<nvdsl::NvdslSpec>;
#[doc = "Non-volatile data segment length"]
pub mod nvdsl;
#[doc = "VDSSA (rw) register accessor: Volatile data segment start address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdssa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdssa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdssa`] module"]
#[doc(alias = "VDSSA")]
pub type Vdssa = crate::Reg<vdssa::VdssaSpec>;
#[doc = "Volatile data segment start address"]
pub mod vdssa;
#[doc = "VDSL (rw) register accessor: Volatile data segment length\n\nYou can [`read`](crate::Reg::read) this register and get [`vdsl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdsl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdsl`] module"]
#[doc(alias = "VDSL")]
pub type Vdsl = crate::Reg<vdsl::VdslSpec>;
#[doc = "Volatile data segment length"]
pub mod vdsl;
#[doc = "CR (rw) register accessor: Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Configuration register"]
pub mod cr;
