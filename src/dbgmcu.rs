#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    idcode: Idcode,
    cr: Cr,
    apb1_fzr1: Apb1Fzr1,
    apb1_fzr2: Apb1Fzr2,
    apb2_fzr: Apb2Fzr,
}
impl RegisterBlock {
    #[doc = "0x00 - MCU Device ID Code Register"]
    #[inline(always)]
    pub const fn idcode(&self) -> &Idcode {
        &self.idcode
    }
    #[doc = "0x04 - Debug MCU Configuration Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x08 - APB Low Freeze Register 1"]
    #[inline(always)]
    pub const fn apb1_fzr1(&self) -> &Apb1Fzr1 {
        &self.apb1_fzr1
    }
    #[doc = "0x0c - APB Low Freeze Register 2"]
    #[inline(always)]
    pub const fn apb1_fzr2(&self) -> &Apb1Fzr2 {
        &self.apb1_fzr2
    }
    #[doc = "0x10 - APB High Freeze Register"]
    #[inline(always)]
    pub const fn apb2_fzr(&self) -> &Apb2Fzr {
        &self.apb2_fzr
    }
}
#[doc = "IDCODE (r) register accessor: MCU Device ID Code Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idcode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idcode`] module"]
#[doc(alias = "IDCODE")]
pub type Idcode = crate::Reg<idcode::IdcodeSpec>;
#[doc = "MCU Device ID Code Register"]
pub mod idcode;
#[doc = "CR (rw) register accessor: Debug MCU Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Debug MCU Configuration Register"]
pub mod cr;
#[doc = "APB1_FZR1 (rw) register accessor: APB Low Freeze Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1_fzr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1_fzr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1_fzr1`] module"]
#[doc(alias = "APB1_FZR1")]
pub type Apb1Fzr1 = crate::Reg<apb1_fzr1::Apb1Fzr1Spec>;
#[doc = "APB Low Freeze Register 1"]
pub mod apb1_fzr1;
#[doc = "APB1_FZR2 (rw) register accessor: APB Low Freeze Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1_fzr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1_fzr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1_fzr2`] module"]
#[doc(alias = "APB1_FZR2")]
pub type Apb1Fzr2 = crate::Reg<apb1_fzr2::Apb1Fzr2Spec>;
#[doc = "APB Low Freeze Register 2"]
pub mod apb1_fzr2;
#[doc = "APB2_FZR (rw) register accessor: APB High Freeze Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2_fzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2_fzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2_fzr`] module"]
#[doc(alias = "APB2_FZR")]
pub type Apb2Fzr = crate::Reg<apb2_fzr::Apb2FzrSpec>;
#[doc = "APB High Freeze Register"]
pub mod apb2_fzr;
