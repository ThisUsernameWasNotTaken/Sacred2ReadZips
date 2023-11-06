fn main() {
    let sourceFiles =
        "\\src\\gcl\\bindings\\bonebinding.cpp
\\src\\gcl\\bindings\\animation.cpp
\\src\\gcl\\bindings\\abstractcurvekey.cpp
\\src\\gcl\\bindings\\skeleton.cpp
\\src\\gcl\\bindings\\model.cpp
\\src\\gcl\\bindings\\mesh.cpp
\\src\\gcl\\bindings\\bone.cpp
\\src\\gcl\\bindings\\curvescalekey.cpp
\\src\\gcl\\bindings\\curvepositionkey.cpp
\\src\\gcl\\bindings\\track.cpp
\\src\\gcl\\bindings\\scene.cpp
\\src\\gcl\\bindings\\material.cpp
\\src\\gcl\\exporter\\fbxexporter.cpp
\\src\\gcl\\bindings\\curverotationkey.cpp
\\src\\gcl\\importer\\grannyformat.cpp
\\src\\gcl\\importer\\grannyimportermaterial.cpp
\\src\\gcl\\exporter\\fbxexporteranimation.cpp
\\src\\gcl\\importer\\deboor.cpp
\\src\\gcl\\exporter\\fbxexporterskeleton.cpp
\\src\\gcl\\exporter\\fbxexportermodulefactory.cpp
\\src\\gcl\\grannyconverterlibrary.cpp
\\src\\gcl\\exporter\\fbxexportermodule.cpp
\\src\\gcl\\importer\\grannyimporteranimation.cpp
\\src\\gcl\\exporter\\fbxexportermesh.cpp
\\src\\gcl\\importer\\grannyimporteranimation_deboor.cpp
\\src\\gcl\\importer\\grannyimporter.cpp
\\src\\gcl\\exporter\\fbxexportermaterial.cpp
\\src\\gcl\\utilities\\materialutility.cpp
\\src\\gcl\\utilities\\devilimageutility.cpp
\\src\\gcl\\importer\\grannyimportermodel.cpp
\\src\\gcl\\utilities\\filestreamutility.cpp
\\src\\gcl\\utilities\\stringutility.cpp
\\src\\gcl\\utilities\\datetime.cpp
\\src\\gcl\\utilities\\logging.cpp
\\src\\gcl\\utilities\\textureutility.cpp
\\src\\gcl\\importer\\grannyimporterskeleton.cpp
\\src\\gcl\\utilities\\fbxsdkcommon.cpp";

    let mut build = cc::Build::new();
    let sourceFolder = "E:\\root\\Dateien\\Sacred\\GrannyConverterLibrary";
    for line in sourceFiles.lines() {
        build.file(sourceFolder.to_owned() + line);
    }
    build
        .include("E:\\root\\Dateien\\Sacred\\GrannyConverterLibrary\\external\\devilsdk\\include")
        .include("C:\\Program Files\\Autodesk\\FBX\\FBX SDK\\2020.3.1\\include")
        .include(sourceFolder.to_owned() + "\\out\\build\\x86-Debug")
        .include(sourceFolder.to_owned() + "\\src")
        .include(sourceFolder)
        .std("c++17")
        .compile("granny2converter");

    println!("cargo:rerun-if-changed=src/main");
    println!("cargo:rerun-if-changed=src/blobstore.cc");
    println!("cargo:rerun-if-changed=include/blobstore.h");
}
