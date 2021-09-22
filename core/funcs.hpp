#pragma once
#include <dlapi.h>
#include "utils.hpp"
#include "result.h"

Result<ExposeResult, const char *> expose(dl::ICameraPtr camera, ExposureInfo exp_info);

