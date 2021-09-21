#include <dlapi.h>
#include <vector>
#include "utils.h"
#include "result.h"

Result<ExposeResult, const char *> expose(dl::ICameraPtr camera, ExposureInfo exp_info) {
  auto sensor = camera->getSensor(0);
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

  ExposeResult expose_result;

  auto img = sensor->getImage();
  expose_result.buffer = img->getBufferData();
  expose_result.buffer_size = img->getBufferLength();

  return Ok(expose_result);
}

Result<dl::ICameraPtr, int> initialize_camera(dl::IGatewayPtr gateway) {

	gateway->queryUSBCameras();

	auto count = gateway->getUSBCameraCount();
	if (count == 0) {
		return Err(0);
	}

	auto camera = gateway->getUSBCamera(0);
	if (!camera) {
		return Err(0);	
	}

	camera->initialize();

  return Ok(camera);
}

void free_gateway(dl::IGatewayPtr gateway) {
  dl::deleteGateway(gateway);
}
