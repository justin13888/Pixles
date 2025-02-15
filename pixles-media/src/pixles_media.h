#pragma once

#include <vector>
#include <string>

// TODO: Update
#ifdef _WIN32
  #define PIXLES_MEDIA_EXPORT __declspec(dllexport)
#else
  #define PIXLES_MEDIA_EXPORT
#endif

PIXLES_MEDIA_EXPORT void pixles_media();
PIXLES_MEDIA_EXPORT void pixles_media_print_vector(const std::vector<std::string> &strings);
