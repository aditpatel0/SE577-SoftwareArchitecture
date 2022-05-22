<template>
  <div class="q-pa-md">
    <q-banner class="bg-primary text-white">
      <h6>Block Chain Parent Page</h6>
    </q-banner>
    <div class="row">
      <div class="col-4">
        <BCBasicComponent />
      </div>
      <div class="col-4">
        <BCAdvancedComponent />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import BCAdvancedComponent from 'src/components/BC/BCAdvanced.vue';
import BCBasicComponent from 'src/components/BC/BCBasic.vue';

export default defineComponent({
  name: 'BCPage',
  components: { BCAdvancedComponent, BCBasicComponent },
  setup() {
    return {};
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped></style>
