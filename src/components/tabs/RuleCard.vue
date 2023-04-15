<script setup lang="ts">
    import { PropType } from "vue";
import ToggleButton from "../buttons/ToggleButton.vue";
import FancyButton from "../buttons/FancyButton.vue";

    enum Rules {
        Tax,
        BusinessTax,
        BusinessFunding,
        DenyAge,
        DenyHealthPercentage,
        CoverFood,
        CoverFoodUnemployed,
    }

    interface RuleCardValue {
        startStr?: string;
        value: any;
        endStr?: string;
    }

    type ActivationToggleFn = (activated: boolean) => void;
    type UpdateRuleFn = (data: any[]) => void;

    const props = defineProps({
        category: { type: String, required: true },
        title: { type: String, required: true },
        values: { type: Object as PropType<RuleCardValue[]>, required: true },
        data: { type: Object, required: true },
        enabled: { type: Boolean, required: true },
        onActivationToggle: { type: Object as PropType<ActivationToggleFn> },
        updateRuleFn: { type: Object as PropType<UpdateRuleFn> },
    })

    let showAmendModal = false;
</script>

<template>
<div style="opacity: {enabled ? 1 : 0.5}" class="container">
    <div v-if="showAmendModal">
        <AmendRule
            {updateRuleFn}
            bind:shown={showAmendModal}
            updateValues={values}
        />
    </div>

    <div class="left">
        <div>
            <h5>{{category.toUpperCase()}}</h5>
            <h4>{{title}}</h4>
        </div>

        <div>
            <p>
                <div v-for="({ value }, key) in data">
                    {{ value.startStr || "" }}<strong>{{ value }}</strong>
                </div>
            </p>
        </div>

        <div>
            <ToggleButton
                activeText="Deactivate"
                inactiveText="Activate"
                bind:isToggled={enabled}
                width="200px"
            /> <!-- BEEPBOOP(conaticus): idek if this will work -->
            <FancyButton
                bind:enabled
                @click="() => showAmendModal = true"
                width="200px">Amend</FancyButton> <!-- BEEPBOOP(conaticus): Idk if this is the right button lol -->
            >
        </div>
    </div>

    <div class="right">
        <div class="keys">
            <p v-for="(_, key) in data">{{key}}:&nbsp;</p> <!-- chinese -->
        </div>

        <div class="values">
            <p v-for="(value) in data">{{value}}</p> <!-- polish (idk anymore im losing the plot, delerium) -->
        </div>
    </div>
</div>
</template>

<style scoped>
    .container {
        display: flex;
        border: solid 1px var(--color-accent);
        margin: 20px;
        padding: 10px;
        height: 120px;
    }
    .container .left {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        margin-right: 50px;
    }
    .container .left div {
        display: flex;
    }
    h5 {
        background-color: var(--color-accent);
        color: black;
        padding: 5px 40px;
        font-weight: bold;
        font-size: 16px;
        margin-right: 20px;
    }
    h4 {
        font-size: 20px;
        padding: 5px;
        font-weight: 500;
        color: grey;
    }
    .left p {
        font-size: 20px;
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
</style>