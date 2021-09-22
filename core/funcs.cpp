#include <dlapi.h>
#include <fitsio.h>
#include <vector>
#include "utils.hpp"
#include "result.h"

Result<dl::IImagePtr, const char *> expose(dl::ICameraPtr camera, dl::ISensorPtr sensor, ExposureInfo exp_info) {
  auto sensor_info = sensor->getInfo();

  dl::TSubframe subframe; 
  subframe.top = 0;
  subframe.left = 0;
  subframe.width = sensor_info.pixelSizeX;
  subframe.height = sensor_info.pixelSizeY;
  subframe.binX = exp_info.bin_x;
  subframe.binY = exp_info.bin_y;

  dl::TExposureOptions exposure_options;
  exposure_options.duration = exp_info.duration;
  exposure_options.binX = 1;
	exposure_options.binY = 1;
	exposure_options.readoutMode = static_cast<int>(exp_info.readout_mode);
	exposure_options.isLightFrame = exp_info.light;
	exposure_options.useRBIPreflash = false;
	exposure_options.useExtTrigger = false;

	try {
		await(sensor->setSubframe(subframe));
	} catch (std::exception &ex) {
		return Err(ex.what());
	}

  // start exposure
  try {
    await(sensor->startExposure(exposure_options));
  } catch (std::exception &ex) {
    return Err(ex.what());
  }

  // wait for exposure to finish
  do {
    try {
      await(camera->queryStatus());	
    } catch (std::exception &ex) {
      return Err(ex.what());
    }
    auto status = camera->getStatus();
    if (status.mainSensorState == dl::ISensor::ReadyToDownload) break;
  } while (true);

  // get data
  try {
    await(sensor->startDownload());
  } catch (std::exception &ex) {
    return Err(ex.what());
  }

  auto img = sensor->getImage();

  return Ok(img);
}

void save_image(dl::IImagePtr image, char *filepath) {

  unsigned short * buffer = image->getBufferData();
  long nelements = image->getBufferLength();
  auto metadata = image->getMetadata();

  fitsfile *fptr;
  int status = 0;
  int fpixel = 1;
  long naxis = 2;
  long naxes[2] = { metadata.width, metadata.height };
  int bitpix = USHORT_IMG;
  float duration = metadata.exposureDuration;

  fits_create_file(&fptr, filepath, &status);
  print_fits_err(status);
  fits_create_img(fptr, bitpix, naxis, naxes, &status);
  print_fits_err(status);
  fits_write_img(fptr, TUSHORT, fpixel, nelements, buffer, &status);
  print_fits_err(status);
  fits_update_key(fptr, TLONG, "EXPOSURE", &duration, "Total exposure time", &status);
  print_fits_err(status);
  fits_close_file(fptr, &status);
  print_fits_err(status);
}
