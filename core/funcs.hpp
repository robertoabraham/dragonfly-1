#pragma once
#include <dlapi.h>
#include "utils.hpp"
#include "result.h"

Result<ExposeResult, const char *> expose(dl::ICameraPtr camera, dl::ISensorPtr sensor, ExposureInfo exp_info);

