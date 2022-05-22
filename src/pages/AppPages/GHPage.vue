<template>
  <div class="q-pa-md">
    <q-banner class="bg-primary text-white">
      <h6>GitHubDemo</h6>
    </q-banner>
    <div class="row">
      <q-table
        title="GitHub Data"
        dense
        :rows="rows"
        :columns="columns"
        row-key="id"
      />
    </div>
  </div>
</template>

<script lang="ts">
export default {
  name: 'GitHubPage',
};
</script>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import axios from 'axios';

type rowType = {
  id: string;
  name: string;
  url: string;
  language: string;
  updated_at: string;
};

const columns = [
  { name: 'id', label: 'ID', align: 'left', field: 'id', sortable: true },
  { name: 'name', label: 'name', align: 'left', field: 'name', sortable: true },
  { name: 'url', label: 'URL', align: 'left', field: 'url', sortable: true },
  { name: 'language', label: 'Language', align: 'left', field: 'language' },
  { name: 'updated_at', label: 'Updated', align: 'left', field: 'updated_at' },
];
let rows = ref([] as rowType[]);

onMounted(async () => {
  const res = await axios.get(
    'https://api.github.com/users/architectingsoftware/repos'
  );

  rows.value = [];
  const rList = res.data as rowType[];
  const resList = rList.map((row) => {
    const mappedRow: rowType = {
      id: row.id,
      name: row.name,
      url: row.url,
      language: row.language,
      updated_at: row.updated_at,
    };
    return mappedRow;
  });
  console.log('DEBUG', resList);
  rows.value = resList;
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped></style>
