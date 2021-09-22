#include <dlapi.h>
#include "utils.hpp"

SensorInfo sensor_info(dl::ISensorPtr sensor) {
  auto info = sensor->getInfo();

  SensorInfo sensor_info;

  sensor_info.pixels_x = info.pixelsX;
  sensor_info.pixels_y = info.pixelsY;
  sensor_info.pixel_size_x = info.pixelSizeX;
  sensor_info.pixel_size_y = info.pixelSizeY;
  sensor_info.cooler_setpoint_min = info.minCoolerSetpoint;
  sensor_info.cooler_setpoint_max = info.maxCoolerSetpoint;
  sensor_info.bin_x_max = info.maxBinX;
  sensor_info.bin_y_max = info.maxBinY;
  sensor_info.exposure_duration_min  = info.minExposureDuration;
  sensor_info.exposure_precision = info.exposurePrecision;

  return sensor_info;
}
