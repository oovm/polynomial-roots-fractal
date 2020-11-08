(* ::Package:: *)

roots[c_List] := Block[
	{a = DiagonalMatrix[ConstantArray[1., Length@c - 1], -1]},
	a[[1]] = -c;
	Eigenvalues[a]
];

export[rank_] := Block[
	{data = ReIm /@ Flatten@ParallelMap[roots, Tuples[{-1., 1.}, rank]]},
	Export["polynomial_roots_" <> ToString[rank] <> ".wxf", data, PerformanceGoal -> "Size"]
];


export /@ Range[2, 12] // AbsoluteTiming


export[13] // AbsoluteTiming
export[14] // AbsoluteTiming
export[15] // AbsoluteTiming
export[16] // AbsoluteTiming


export[17] // AbsoluteTiming
export[18] // AbsoluteTiming
export[19] // AbsoluteTiming
export[20] // AbsoluteTiming
