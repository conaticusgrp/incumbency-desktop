<script setup lang="ts">
import Window from "./Window.vue";
import {
  TAB_LIST_ENTRY_MARGIN,
  TAB_LIST_MIN_WIDTH,
  TAB_LIST_WIDTH,
  USERNAME,
  USERNAME_HEIGHT,
} from "../../constants";
import { ref, useSlots } from "vue";
import TabButton from "../buttons/TabButton.vue";

defineProps<{
  title: string;
  pos: Pos;
  size: Size;
}>();
const slots = useSlots();
const slotArray = Object.keys(slots);
const currentTabI = ref(0);

const onTabSelect = (newTabI: number) => () => {
  currentTabI.value = newTabI;
}

// styles
const windowStyle = `
  --tab-list-width: ${TAB_LIST_WIDTH};
  --tab-list-min-width: ${TAB_LIST_MIN_WIDTH};
  --tab-list-entry-margin: ${TAB_LIST_ENTRY_MARGIN};
  --username-height: ${USERNAME_HEIGHT};
`;
</script>

<template>
  <Window {title} {pos} {size} {windowData}>
    <div :style=windowStyle>
      <section>
        <div class="tab-list">
          <div v-for="tabName, i in slotArray">
            <TabButton :selected="i === currentTabI" @select-tab=onTabSelect(i)>
              {{ tabName }}
            </TabButton>
          </div>
        </div>

        <div class="username">
          Authenticated as:
          <div>{{ USERNAME }}</div>
        </div>
      </section>

      <section>
        <div v-for="slotName, i in slotArray">
          <slot :name=slotName v-if="i === currentTabI"></slot>
        </div>
      </section>
    </div>
  </Window>
</template>

<style scoped>
main {
  display: flex;
  flex-direction: row;
  width: 100%;
  height: 100%;
}

main>section:first-of-type {
  width: var(--tab-list-width);
  min-width: var(--tab-list-min-width);
  height: 100%;
  border-right: 1px solid var(--color-accent);
}

.tab-list {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: calc(100% - var(--username-height));
  overflow-y: scroll;
}

.tab-list::-webkit-scrollbar {
  display: none;
}

.username {
  width: 100%;
  height: var(--username-height);
  border-top: 1px solid var(--color-accent);
  color: var(--color-accent);
}

.username div {
  font-weight: bold;
}

main>section:last-of-type {
  width: calc(100% - max(var(--tab-list-width), var(--tab-list-min-width)));
}
</style>
