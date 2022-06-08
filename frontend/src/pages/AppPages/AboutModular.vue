<template>
  <div class="q-pa-md">
    <AboutSection />
    <BackgroundSection />
    <CourseSection />
    <PublicationsSection />
    <PhotosSection />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import AboutSection from 'components/About/AboutSection.vue';
import BackgroundSection from 'components/About/BackgroundSection.vue';
import CourseSection from 'components/About/CourseSection.vue';
import PublicationsSection from 'components/About/PublicationsSection.vue';
import PhotosSection from 'components/About/PhotosSection.vue';

export default defineComponent({
  name: 'AboutModularPage',
  components: {
    AboutSection,
    BackgroundSection,
    CourseSection,
    PublicationsSection,
    PhotosSection,
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped></style>
