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


gif[i_, j_] := ImageResize[draw[Import["polynomial_roots_" <> ToString[i] <> ".wxf"], j], 200];

Export["roots.gif",
	{
		gif[2, 100],
		gif[3, 100],
		gif[4, 100],
		gif[5, 100],
		gif[6, 200],
		gif[7, 200],
		gif[8, 200],
		gif[9, 200]
	},
	"DisplayDurations" -> ConstantArray[1, 8]
]
