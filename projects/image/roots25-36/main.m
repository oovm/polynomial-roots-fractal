(* ::Package:: *)

SetDirectory@NotebookDirectory[];


loadData[i_]:=extend/@Normal@Import["C:\\Users\\Dell\\CLionProjects\\polynomial-roots-fractal\\target\\PolynomialRoots\\littlewood\\complex_"<>ToString[i]<>".wxf"];
extend[Complex[a_,b_]]:=If[a>4||b>4, Nothing,{a +b I ,a-b  I, -a +b I , -a-b I}];
With[
{\[Gamma] = 0.12, \[Beta] = 1.},
     fLor = Compile[{{x, _Integer}, {y, _Integer}},(\[Gamma]/(\[Gamma] + x^2 + y^2))^\[Beta], RuntimeAttributes -> {Listable}]
];

PlotComplexPoints[list_, magnification_, paddingX_, paddingY_, brightness_, vec_] := Module[
    {dimX, dimY, RePos, ImPos, lor, posf, sparse},
           posf = 1 + Round[magnification (# - Min[#])] &;
           RePos = paddingX + posf[Re[list]]; ImPos = paddingY + posf[Im[list]];
           dimX = paddingX + Max[RePos]; dimY = paddingY + Max[ImPos];
           With[
    {spopt = SystemOptions["SparseArrayOptions"]},
                Internal`WithLocalSettings[
                SetSystemOptions["SparseArrayOptions" -> {"TreatRepeatedEntries" -> 1}],
                Image[Outer[Times,
                            brightness Abs[InverseFourier[Fourier[
                            SparseArray[Thread[Transpose[{ImPos, RePos}] -> 
                                        ConstantArray[1, Length[list]]], {dimY, dimX}]]
                            Fourier[RotateRight[fLor[#[[All, All, 1]],
                                                     #[[All, All, 2]]] & @
                                    Outer[List,
Range[-Quotient[dimY, 2],Quotient[dimY - 1, 2]], 
                                          Range[-Quotient[dimX, 2], Quotient[dimX - 1, 2]]],
                                          {Quotient[dimY, 2], Quotient[dimX, 2]}]],
                                    FourierParameters -> {-1, 1}]
], 
                            Developer`ToPackedArray[N[vec]]], Magnification -> 1
],
                SetSystemOptions[spopt]
]
]
]


TakeLargest[#,10]&/@Transpose[ReIm/@Normal@Import["C:\\Users\\Dell\\CLionProjects\\polynomial-roots-fractal\\target\\PolynomialRoots\\littlewood\\complex_20.wxf"]]


data=Flatten@Table[loadData[i],{i,17}];
image=PlotComplexPoints[data,5000, 20, 20, 4, {0.1, 0.3, 0.9}];
Export["image.png",image]
