<template>
  <div>
    <h4>Vue Feature Page - Dynamic Behavior</h4>

    <div class="q-pa-md">
      <q-list bordered class="rounded-borders">
        <!-- BINDING EXPANSION -->
        <q-expansion-item
          dark
          expand-separator
          header-class="bg-teal text-white"
          label="Binding"
        >
          <q-card>
            <q-card-section>
              <q-input outlined v-model="bindingText" label="Enter some text" />
              <h5><strong>Entered Text: </strong> {{ bindingText }}</h5>
            </q-card-section>
          </q-card>
        </q-expansion-item>
        <hr />
        <q-expansion-item
          expand-separator
          dark
          header-class="bg-teal text-white"
          label="Show / Hide"
          label-lines="1"
          caption="Will show hide things existing based on condition"
        >
          <q-card>
            <q-card-section>
              <q-toggle
                color="teal"
                v-model="showMeToggle"
                label="Click Me"
                left-label
              />
              <h5 v-show="showMeToggle">
                <strong>This is some Hidden Stuff</strong>
              </h5>
              <hr />
              <div>
                <q-btn
                  @click="buttonPressToggle = !buttonPressToggle"
                  :color="buttonPressToggle ? 'teal' : 'white'"
                  :text-color="!buttonPressToggle ? 'black' : 'white'"
                  label="Click Me"
                />
                <div>
                  <h5 v-show="buttonPressToggle">
                    <strong>Showing some hidden stuff</strong>
                  </h5>
                </div>
              </div>
            </q-card-section>
          </q-card>
        </q-expansion-item>
        <hr />
        <q-expansion-item
          expand-separator
          dark
          header-class="bg-teal text-white"
          label="Conditional Render"
          caption="Will bring code in based on a condition"
        >
          <q-card>
            <q-card-section>
              <q-toggle
                color="teal"
                v-model="crToggle"
                label="Render Me"
                left-label
              />
              <strong>(Checked: </strong> {{ crToggle }} <strong>)</strong>
              <h5 v-if="crToggle">
                <strong>This is some Hidden Stuff</strong>
              </h5>
              <hr />
              <div>
                <q-btn
                  @click="crPressToggle = !crPressToggle"
                  :color="crPressToggle ? 'teal' : 'white'"
                  :text-color="!crPressToggle ? 'black' : 'white'"
                  label="Click Me"
                />
                <div>
                  <h5 v-if="crPressToggle">
                    <strong>Showing some hidden stuff</strong>
                  </h5>
                </div>
                <hr />
                <q-toggle
                  color="teal"
                  v-model="crToggleIfElse"
                  label="If else demo render"
                  left-label
                />
                <strong>(Checked: </strong> {{ crToggleIfElse }}
                <strong>)</strong>
                <h5 v-if="crToggleIfElse">
                  <strong>This Happens when TRUE</strong>
                </h5>
                <h5 v-else>
                  <strong>This happens when FALSE</strong>
                </h5>
              </div>
            </q-card-section>
          </q-card>
        </q-expansion-item>
        <hr />
        <q-expansion-item
          expand-separator
          dark
          header-class="bg-teal text-white"
          label="Looping"
        >
          <q-card>
            <q-card-section>
              <div class="row justify-center">
                <q-input
                  bottom-slots
                  v-model="howManyLoop"
                  label="How Many"
                  :dense="true"
                  type="number"
                  v-model.number="howManyLoop"
                >
                  <template v-slot:prepend>
                    <q-icon name="question_mark" />
                  </template>
                  <template v-slot:append>
                    <q-icon
                      name="close"
                      class="cursor-pointer"
                      @click="clearPressFn"
                    />
                  </template>

                  <template v-slot:after>
                    <q-btn
                      round
                      dense
                      flat
                      icon="replay"
                      @click="loopPressFn"
                    />
                  </template>
                </q-input>
              </div>
              <div class="row justify-left">
                <div
                  class="col-12 q-pb-sm"
                  v-for="(i, index) in howManyRender"
                  :key="i"
                >
                  <q-banner class="bg-primary text-white">
                    Looping at item {{ i }} - index is {{ index }}
                  </q-banner>
                </div>
                <div class="row justify-center">
                  <h6>Total Rendered: {{ howManyRender }}</h6>
                </div>
              </div>
            </q-card-section>
          </q-card>
        </q-expansion-item>
        <hr />
        <q-expansion-item
          expand-separator
          dark
          header-class="bg-teal text-white"
          label="Reactive and Watching"
        >
          <q-card>
            <q-card-section>
              <div class="row justify-center">
                <q-input
                  bottom-slots
                  v-model="reactLoop"
                  label="How Many"
                  :dense="true"
                  type="number"
                  v-model.number="reactLoop"
                >
                  <template v-slot:prepend>
                    <q-icon name="question_mark" />
                  </template>
                  <template v-slot:append>
                    <q-icon
                      name="close"
                      class="cursor-pointer"
                      @click="reactLoop = 0"
                    />
                  </template>
                </q-input>
              </div>
              <div class="row justify-left">
                <div
                  class="col-12 q-pb-sm"
                  v-for="(i, index) in reactLoop"
                  :key="i"
                >
                  <q-banner class="bg-primary text-white">
                    Looping at item {{ i }} - index is {{ index }}
                  </q-banner>
                </div>
                <div class="row justify-center">
                  <h6>Total Rendered: {{ reactLoop }}</h6>
                </div>
              </div>
            </q-card-section>
          </q-card>
        </q-expansion-item>
      </q-list>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { ref } from 'vue';

const BindingSetup = () => {
  return {
    bindingText: ref(''),
  };
};

const ShowHideSetup = () => {
  return {
    showMeToggle: ref(false),
    buttonPressToggle: ref(false),
  };
};

const ConditionalRenderSetup = () => {
  return {
    crToggle: ref(false),
    crPressToggle: ref(false),
    crToggleIfElse: ref(false),
  };
};

const LoopingSetup = () => {
  const howManyLoop = ref(0);
  const howManyRender = ref(howManyLoop.value);
  return {
    howManyLoop,
    howManyRender,
    loopPressFn: () => {
      howManyRender.value = howManyLoop.value;
    },
    clearPressFn: () => {
      howManyRender.value = 0;
      howManyLoop.value = 0;
    },
  };
};

const ReactSetup = () => {
  const reactLoop = ref(0);

  return {
    reactLoop,
  };
};

export default defineComponent({
  name: 'DynamicPage',
  setup() {
    const { bindingText } = BindingSetup();
    const { showMeToggle, buttonPressToggle } = ShowHideSetup();
    const { crToggle, crPressToggle, crToggleIfElse } =
      ConditionalRenderSetup();
    const { howManyLoop, howManyRender, clearPressFn, loopPressFn } =
      LoopingSetup();

    const { reactLoop } = ReactSetup();

    return {
      bindingText,
      showMeToggle,
      buttonPressToggle,
      crToggle,
      crPressToggle,
      crToggleIfElse,
      howManyLoop,
      howManyRender,
      reactLoop,

      loopPressFn,
      clearPressFn,
    };
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped></style>
