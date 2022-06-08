<template>
  <div class="q-pa-md">
    <q-banner class="bg-primary text-white">
      <h6>Im in the Parent Component</h6>
    </q-banner>
    <div class="row">
      <PropsChild class="col-4 q-pa-sm" title="HELLO"></PropsChild>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { ChildCardType } from 'src/models/AppTypes';

import PropsChild from 'components/Props/PropsChild.vue';

const childCardData: ChildCardType[] = [
  {
    name: 'First Card',
    info: 'Card 1 Information',
    link1: 'Link A1',
    link2: 'Link A2',
  },
  {
    name: 'Second Card',
    info: 'Card 2 Information',
    link1: 'Link B1',
    link2: 'Link B2',
  },
  {
    name: 'Third Card',
    info: 'Card 3 Information',
    link1: 'Link C1',
    link2: 'Link C2',
  },
  {
    name: 'Fourth Card',
    info: 'Card 4 Information',
    link1: 'Link D1',
    link2: 'Link D2',
  },
  {
    name: 'Fifth Card',
    info: 'Card 5 Information',
    link1: 'Link E1',
    link2: 'Link E2',
  },
];

export default defineComponent({
  name: 'PropsPage',
  components: { PropsChild },
  setup() {
    return { childCardList: ref(childCardData) };
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped></style>
