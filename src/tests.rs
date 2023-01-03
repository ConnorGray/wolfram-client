use wolfram_expr::{Expr, Symbol};

use crate::{
	EvaluationData, EvaluationOutcome, Output, Packet, PacketExpr,
	WolframSession,
};

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

//======================================
// Test blocking evaluation methods
//======================================

#[test]
fn test_evaluate_text() {
	let mut kernel = WolframSession::launch_default_kernel().unwrap();

	let Packet::InputName(_) = kernel.packets().next().unwrap() else { panic!() };

	let returned = kernel.enter_and_wait("2 + 2").outcome.unwrap_returned();

	assert_eq!(returned, PacketExpr::Text("4".to_owned()));
}

#[test]
fn test_evaluate_expr() {
	let mut kernel = WolframSession::launch_default_kernel().unwrap();

	let Packet::InputName(_) = kernel.packets().next().unwrap() else { panic!() };

	let returned = kernel
		.enter_and_wait(Expr::normal(
			Symbol::new("System`Plus"),
			vec![Expr::from(2), Expr::from(2)],
		))
		.outcome
		.unwrap_returned();

	assert_eq!(returned, PacketExpr::Expr(Expr::from(4)));
}

#[test]
fn test_evaluate_exit() {
	let mut kernel = WolframSession::launch_default_kernel().unwrap();

	let Packet::InputName(_) = kernel.packets().next().unwrap() else { panic!() };

	let outcome = kernel.enter_and_wait("Exit[]").outcome;

	assert_eq!(outcome, EvaluationOutcome::KernelQuit);
}

#[test]
fn test_evaluate_exit_with_epilog() {
	let mut kernel = WolframSession::launch_default_kernel().unwrap();

	let Packet::InputName(_) = kernel.packets().next().unwrap() else { panic!() };

	// Set $Epilog
	kernel
		.enter_and_wait(r#"$Epilog := (Print["Exiting!"]; "dead")"#)
		.outcome
		.unwrap_null();

	// Evaluate `Exit[]`
	let EvaluationData { outcome, output } = kernel.enter_and_wait("Exit[]");

	assert_eq!(
		output,
		vec![Output::Print(PacketExpr::Text("Exiting!\n".to_owned()))]
	);
	assert_eq!(outcome, EvaluationOutcome::KernelQuit);
}

//======================================
// Uncategorized
//======================================
