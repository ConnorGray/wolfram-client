use crate::{Packet, WolframSession};

#[test]
fn test_kernel() {
	let mut kernel = WolframSession::launch_default_kernel().unwrap();

	assert!(matches!(
		kernel.packets().next().unwrap(),
		Packet::InputName(s) if s == "In[1]:= "
	));

	kernel.enter_text("2 + 2");

	assert!(matches!(
		kernel.packets().next().unwrap(),
		Packet::OutputName(s) if s == "Out[1]= "
	));

	assert!(matches!(
		kernel.packets().next().unwrap(),
		Packet::ReturnText(s) if s == "4"
	));

	assert!(matches!(
		kernel.packets().next().unwrap(),
		Packet::InputName(s) if s == "In[2]:= "
	));
}
