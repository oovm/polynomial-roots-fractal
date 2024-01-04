(* ::Package:: *)

SetDirectory@NotebookDirectory[];


Options[draw] = {
	Quantile -> 0.975,
	ColorFunction -> "AvocadoColors"
};
draw[data_, canvas_, o : OptionsPattern[]] := Block[
	{l, group, cut, pixel, plot},
	l = ConstantArray[0., {canvas + 1, canvas + 1}];
	group = Tally@ Round[1 + canvas * Rescale@data];
	cut = Quantile[Last /@ group, OptionValue[Quantile]];
	pixel[{{x_, y_}, v_}] := (l[[x, y]] += If[v > cut, cut, v]);
	pixel /@ group;
	plot = ArrayPlot[l, ColorFunction -> OptionValue[ColorFunction], Frame -> False];
	ImageRotate[Rasterize[plot, "Image", ImageSize -> canvas], Pi / 2]
];



data = Import@"polynomial_roots_20.wxf";
canvas = 2000;
color = "SunsetColors";
img = draw[data, canvas, ColorFunction -> Function[{a}, If[a == 0, Black, ColorData[color][a]]]]
Export["roots-" <> color <> ".png", img];


data = Import@"polynomial_roots_20.wxf";
canvas = 2000;
color = "AvocadoColors";
img = draw[data, canvas, ColorFunction -> Function[{a}, If[a == 0, Black, ColorData[color][a]]]]
Export["roots-" <> color <> ".png", img];


data = Import@"polynomial_roots_20.wxf";
canvas = 2000;
color = "DeepSeaColors";
img = draw[data, canvas, ColorFunction -> Function[{a}, If[a == 0, White, ColorData[color][a]]]]
Export["roots-" <> color <> ".png", img];


data = Import@"polynomial_roots_20.wxf";
canvas = 2000;
color = "GrayTones";
img = draw[data, canvas, ColorFunction -> Function[{a}, If[a == 0, White, ColorData[color][a]]]]
Export["roots-" <> color <> ".png", img];
