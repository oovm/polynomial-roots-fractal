(* ::Package:: *)

SetDirectory@NotebookDirectory[];


rank = 24;
tasks = Tuples[{-1., 1.}, rank];
tasks = Partition[tasks, UpTo[10^5]];


(* ::Subsubsection:: *)
(*Stage 1*)


roots[c_List] := Block[
	{a = DiagonalMatrix[ConstantArray[1., Length@c - 1], -1]},
	a[[1]] = -c;
	Eigenvalues[a]
];
export1[tasks_, {i_}] := Block[
	{data},
	data = ReIm /@ Flatten@ParallelMap[roots, tasks];
	Export["rescale.wxf", MinMax@Flatten[{data, Import["rescale.wxf"]}]];
	Export["stage_1_part_" <> ToString[i] <> ".wxf", data]
];
Export["rescale.wxf", {0, 0}];
ResourceFunction["MonitorProgress"]@MapIndexed[export1, tasks]


(* ::Subsubsection:: *)
(*Stage 2*)


canvas = 10240;
tasks = FileNames["stage_1_part_*"];
export2[tasks_, {i_}] := Block[
	{data},
	data = Tally@Round[1 + canvas * Rescale[Import[tasks], Import@"rescale.wxf"]];
	Export["stage_2_part_" <> ToString[i] <> ".wxf", data]
];
ResourceFunction["MonitorProgress"]@MapIndexed[export2, tasks]


(* ::Subsubsection:: *)
(*Stage 3*)


cut = 0.975;
data = ResourceFunction["MetaTally"][Import /@ FileNames["stage_2_part_*"]];
cut = Quantile[Last /@ group, cut];
Export["stage_3_group.wxf", data];
Export["stage_3_cut.wxf", cut]
