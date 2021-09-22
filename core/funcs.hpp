#pragma once
#include <dlapi.h>
#include "utils.hpp"
#include "result.h"

Result<dl::IImagePtr, const char *> expose(dl::ICameraPtr camera, dl::ISensorPtr sensor, ExposureInfo exp_info);
void save_image(dl::IImagePtr image, char *filepath);
