#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum LanguageMode{Beg,Adv,AdvN}
#[derive(Debug,Clone,Copy)]
pub struct ModeRules{pub require_sections:bool,pub allow_inline_assembly:bool,pub strict_types:bool}
impl LanguageMode{pub fn rules(self)->ModeRules{match self{Self::Beg=>ModeRules{require_sections:true,allow_inline_assembly:false,strict_types:true},Self::Adv=>ModeRules{require_sections:false,allow_inline_assembly:true,strict_types:true},Self::AdvN=>ModeRules{require_sections:false,allow_inline_assembly:true,strict_types:false}}}}
