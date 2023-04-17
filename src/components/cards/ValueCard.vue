<script setup lang="ts">
import { PropType, ref } from "vue";
import Button from "../buttons/FancyButton.vue";
import AmendModal from "../modals/AmendModal.vue";

const props = defineProps({
  title: { type: String, required: true },
  value: { type: [Number], required: true },
  detail: { type: Object as PropType<{ [key: string]: any }>, required: true },
  hasAmendButton: { type: Boolean, default: true },
});

type Value = typeof props.value;
const emits = defineEmits<{
  (e: "amend", v: Value): void
}>();

const hasAmendButton = ref(props.hasAmendButton);
const showAmendModal = ref(false);

const title = `Enter new value for "${props.title}"`;
const buttonStyle = `--container-height: ${hasAmendButton ? '120px' : '60px'};`;
const setShowAmendModal = (value: boolean) => {
  showAmendModal.value = value;
};
const emitValue = (value?: Value) => {
  setShowAmendModal(!showAmendModal.value);
  if (value) {
    emits("amend", value);
  }
};
</script>

<template>
  <div :style=buttonStyle>
    <AmendModal v-if=showAmendModal :title="title" :value=value @amendCancel="emitValue()"
      @amendConfirm="emitValue($event.valueOf())" />

    <div class="container">
      <div class="left">
        <h4>{{ title }}</h4>
        <h2>{{ value }}</h2>
        <Button v-if="hasAmendButton" @onClick="setShowAmendModal(true)" width="200px">
          Amend
        </Button>
      </div>

      <div class="right">
        <div class="keys">
          <p v-for="key in Object.keys(detail)"> {{ key }}:&nbsp;</p>
        </div>

        <div class="values">
          <p v-for="value in Object.values(detail)">{{ value }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.container {
  width: 95%;
  display: flex;
  margin: 0 auto;
  margin-bottom: 20px;
  padding: 10px;
  height: var(--container-height);
  border: solid 1px var(--color-accent);
}

.container .left {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  margin-right: 20px;
  width: 200px;
}

.container .right {
  display: flex;
  text-align: left;
}

.container .right .keys {
  margin-right: 20px;
}

.container .right p {
  font-size: 15px;
  font-weight: normal;
  margin-bottom: 3px;
}

.container .right .keys p {
  color: grey;
}

h4 {
  font-size: 20px;
  color: grey;
  font-weight: 500;
}

h2 {
  font-size: 30px;
  font-weight: bold;
}
</style>
