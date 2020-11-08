(* ::Package:: *)

SetDirectory@NotebookDirectory[];


Options[draw] = {
	Quantile -> 0.975
};
draw[data_, canvas_, o : OptionsPattern[]] := Block[
	{l = ConstantArray[0., {canvas + 1, canvas + 1}], cutted, plot},
	l[[#, #2]] += 1.& @@@ Round[1 + canvas * Rescale@data];
	cutted = Quantile[DeleteCases[Flatten[l], 0.], OptionValue[Quantile]];
	plot = ArrayPlot[UnitStep[cutted - l]l, ColorFunction -> "AvocadoColors", Frame -> False];
	ImageRotate[ImagePad[plot, -20], Pi / 2]
];


gif[i_, j_] := ImageResize[draw[Import["polynomial_roots_" <> ToString[i] <> ".wxf"], j], 400];

Export["roots.gif",
	{
		gif[9, 400],
		gif[10, 400],
		gif[11, 400],
		gif[12, 400],
		gif[13, 400],
		gif[14, 400],
		gif[15, 400],
		gif[16, 400]
	},
	"DisplayDurations" -> ConstantArray[0.5, 8]
]
