<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { ref } from "vue";

type LoadingStatus = {
  main: string;
  substatuses: string[];
}

type EventPayload = {
  [key: string]: string[];
}

const LOADING_STAGE_COUNT = 4;
const MIN_LOG_DELAY = 250;
const MAX_LOG_DELAY = 500;

const details = ref<HTMLElement | null>(null);
const progress = ref(-100 / LOADING_STAGE_COUNT); // %
const status = ref<LoadingStatus>({ main: "", substatuses: [] });
let timerResolve: (() => void) | null = null;

const randomDelay = async () => {
  await new Promise<void>((resolve) => {
    setTimeout(
      resolve,
      MIN_LOG_DELAY + (MAX_LOG_DELAY - MIN_LOG_DELAY) * Math.random()
    );
  });
};

const log = (msg: string): void => {
  if (details.value) {
    details.value.innerText += `[INFO]  ${msg}\n`;
  }
};

listen<EventPayload>("loading_status", (e) => {
  if (timerResolve != null) {
    timerResolve();
    status.value.substatuses.forEach((m) => log(m));
  }

  status.value.main = Object.keys(e.payload)[0];
  status.value.substatuses = e.payload[status.value.main];
  progress.value += 100 / LOADING_STAGE_COUNT;

  new Promise<void>(async (resolve) => {
    timerResolve = resolve;
    for (let i = 0; i < status.value.substatuses.length; i++) {
      await randomDelay();
      log(status.value.substatuses.splice(i, 1)[0]);
    }
    timerResolve = null;
    resolve();
  });
});
</script>

<template>
  <main>
    <div class="loading-panel">
      <h1>{status.main}</h1>

      <div class="content">
        <div class="progress">
          <span>Progress</span>
          <div class="progress-bar" style="--bar-width: {Math.min(
                            Math.max(progress, 0),
                            100
                        )}%;" />
        </div>

        <span style="margin-top: 2em;">Details</span>
        <div class="details" bind:this={details} />
      </div>
    </div>
  </main>
</template>

<style scoped>
main {
  display: flex;
  align-items: center;
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 100;
  isolation: isolate;
  background-color: var(--color-bg);
}

.loading-panel {
  display: flex;
  flex-direction: column;
  min-width: 400px;
  width: 50%;
  aspect-ratio: 5 / 2;
  margin: auto;
  border: 1px solid var(--color-accent);
  border-radius: 0.5rem;
}

h1 {
  width: calc(100% - 2 * 0.5em);
  padding: 0.25em 0.5em 0.25em 0.5em;
  color: var(--color-bg);
  background-color: var(--color-accent);
  border-top-left-radius: 0.3rem;
  border-top-right-radius: 0.3rem;
  letter-spacing: 0.25em;
  text-align: center;
  text-transform: uppercase;
  white-space: nowrap;
  text-overflow: ellipsis;
  font-weight: bold;
  font-size: 20px;
  text-overflow: ellipsis;
  overflow: hidden;
  white-space: nowrap;
}

.content {
  display: flex;
  flex-direction: column;
  height: 100%;
  margin: 1em;
  text-align: left;
}

.progress-bar {
  position: relative;
  width: 100%;
  height: 5px;
  margin-top: 0.5em;
}

.progress-bar:after {
  content: "";
  position: absolute;
  top: 0;
  left: 0;
  width: var(--bar-width);
  height: 100%;
  background-color: var(--color-accent);
  transition: width 2s ease;
}

.details {
  width: calc(100% - 1em);
  height: calc(100% - 2em - 1em);
  border: 1px solid var(--color-accent);
  border-radius: 1em;
  padding-left: 1em;
  overflow-y: scroll;
  padding-left: 10px;
  color: grey;
}

.details::-webkit-scrollbar {
  display: none;
}
</style>
