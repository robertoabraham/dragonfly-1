#pragma once
#include <dlapi.h>
#include <vector>
#include "utils.h"
#include "result.h"

Result<ExposeResult, const char *> expose(dl::ICameraPtr camera, ExposureInfo exp_info);

Result<dl::ICameraPtr, int> initialize_camera(dl::IGatewayPtr gateway);

void free_gateway(dl::IGatewayPtr gateway);
