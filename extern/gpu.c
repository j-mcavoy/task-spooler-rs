//
// Created by justanhduc on 11/10/20.
//

#include <stdlib.h>
//#include <nvml.h>
#include <string.h>

#include "main.h"

#define TS_VISIBLE_DEVICES "TS_VISIBLE_DEVICES"

static int free_percentage = 90;
static int num_total_gpus;
static int *used_gpus = 0;

static void set_cuda_env() {
    unsetenv("CUDA_VISIBLE_DEVICES");
    setenv("CUDA_DEVICE_ORDER", "PCI_BUS_I", 1);
}

void initGPU() {
}

static int getVisibleGpus(int *visibility) {
}

int * getGpuList(int *num) {
}

void broadcastUsedGpus(int num, const int *list) {
}

void broadcastFreeGpus(int num, const int *list) {
}

int isInUse(int id) {
}

void setFreePercentage(int percent) {
}

int getFreePercentage() {
}

void cleanupGpu() {
}
