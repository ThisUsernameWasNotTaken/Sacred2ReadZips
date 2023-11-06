#include "gcl/exporter/fbxexporter.h"
#include "gcl/exporter/fbxexportoptions.h"
#include "gcl/grannyconverterlibrary.h"
#include "gcl/importer/grannyimporter.h"
#include "gcl/importer/grannyimportoptions.h"
#include "gcl/utilities/logging.h"

#include <stdio.h>  /* defines FILENAME_MAX */
#include <string>
#include <iostream>
#include <filesystem>
#include <direct.h>
#include "main.h"

#define GetCurrentDir _getcwd

/*
note that the converter will overwrite the bones of the current model with ALL getBones() of all other models in a loop, if itself doesnt have any. was probably done to auto-solve animations beeing split off in seperate files, but will obviously and DID cause errors and confusion while converting multiple files. the behaviour will be user controlled in the future.
*/

bool has_suffix(const std::string& str, const std::string& suffix)
{
	bool x = str.size() >= suffix.size() && str.compare(str.size() - suffix.size(), suffix.size(), suffix) == 0;
	return x;
}

int main()
{
	char cCurrentPath[FILENAME_MAX];

	auto folderPath = GetCurrentDir(cCurrentPath, sizeof(cCurrentPath));
	GCL::Utilities::Logging::info("%s", folderPath);
	folderPath = "E:\\root\\Dateien\\Sacred\\RustReadZips\\sacred extract test\\";

	std::string baseFilename = "graphics05.zipa_helve.GR2";//"E:\\root\\Dateien\\Sacred\\RustReadZips\\sacred extract test\\graphics05.zipa_helve.GR2";
	std::string baseFilepath; // = "E:\\root\\Dateien\\Sacred\\RustReadZips\\sacred extract test\\graphics05.zipa_helve.GR2";
	std::vector<std::string> list;
	for (const auto& entry : filesystem::directory_iterator(folderPath))
	{
		std::string filePath = entry.path().string();
		size_t findPos = filePath.find(baseFilename);
		size_t endPos = filePath.length() - baseFilename.length();
		if (findPos != std::string::npos && findPos >= endPos)
			baseFilepath = filePath;
		else if ((has_suffix(filePath, ".gr2") || has_suffix(filePath, ".GR2")))
			list.push_back(filePath);
		else
			;
	}
	//list.push_back("E:\\root\\Dateien\\Sacred\\RustReadZips\\sacred extract test\\graphics05.zipa_helve_walk_thr2.GR2");

	extractFBX(baseFilepath, list);

	return 0;
}


int extractFBX(std::string& baseFilepath, std::vector<std::string>& list)
{
	// Initialize library.
	GCL::GrannyConverterLibrary grannyConverterLibrary;

	GCL::Importer::GrannyImportOptions options;

	// Use deboor animation importer.
	// The importer is able to import animations with bones being mis-positioned.
	// However the resulting animation might be changed in comparison to its original.
	// options.importAnimationDeboor = true;

	// Initialize importer.
	GCL::Importer::GrannyImporter importer(options);

	// Load a character and a animation.
	importer.importFromFile(baseFilepath.c_str());
	for (size_t i = 0; i < list.size(); i++)
	{
		// todo test if this is indeed what it looks like:
		string iFilepath = list.at(i);
		importer.importFromFile(iFilepath.c_str());
		// // if i can import multiple .gr2 file "on top of each other" like layers and then export all into one .fbx then the animations would be solved.
		// // and if this wont work look at this deboor feature above
		// (although i still dont know how converting back to .gr2 will play out...)
	}

	GCL::Exporter::FbxExportOptions exporterOptions;

	// Export all skeletons.
	exporterOptions.exportSkeleton = true;

	// Export all materials and textures.
	exporterOptions.exportMaterials = false;

	// Export animations.
	exporterOptions.exportAnimation = true;

	// Create exporter instance with the scene to be exported.
	GCL::Exporter::FbxExporter exporter(exporterOptions, importer.getScene());

	// Export the fbx scene to a fbx file.
	exporter.exportToFile(baseFilepath + ".fbx");

	return 0;
}
