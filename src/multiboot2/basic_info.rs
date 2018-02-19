//basic_info.rs
//contains structs and methods for the basic memory information tag (type 4)

#[repr(packed)]
pub struct BasicMemoryInfoTag {
	tag_type: u32,
	size: u32,
	pub mem_lower: u32,
	pub mem_upper: u32,
}
