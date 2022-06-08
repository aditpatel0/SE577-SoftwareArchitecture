<template>
  <div class="q-pa-md">
    <q-banner class="bg-primary text-green">
      <h6>GitHubDemo</h6>
    </q-banner>
    div class="repoTable">
      <q-table
        title="Repositories"
        dense
        :rows="repoRows"
        :columns="repoColumns"
        row-key="id"
        @row-click="onRepoRowClick"
      />
    </div>
    <div class="branchTable">
      <q-table
        title="Branches"
        dense
        :rows="branchRows"
        :columns="branchColumns"
        row-key="sha"
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


const onRepoRowClick = async (evt, row) => {
    const res = await axios.get(`http://localhost:3000/${row.name}/branches`);
    branchRows.value = [];
    const branches = res.data as branchRowType[];
    let branchDisplayResults = branches.map((row) => {
      const mappedRow: branchRowDisplayType = {
        name: row.name,
        sha: row.commit.sha,
        url: row.commit.url,
        protected: row.protected,
      };
      return mappedRow;
    });
    branchRows.value = branchDisplayResults;
};

type repoRowType = {
  id: string;
  name: string;
  html_url: string;
  language: string;
  updated_at: string;
};
type branchRowType = {
  name: string;
  commit: {
    sha: string;
    url: string;
  }
  protected: boolean;
};
type branchRowDisplayType = {
  name: string;
  sha: string;
  url: string;
  protected: boolean;
}
const repoColumns = [
  { name: 'id', label: 'ID', align: 'left', field: 'id', sortable: true },
  { name: 'name', label: 'name', align: 'left', field: 'name', sortable: true },
  { name: 'url', label: 'URL', align: 'left', field: 'url', sortable: true },
  { name: 'description', label: 'Description', align: 'left', field: 'description' },
  { name: 'language', label: 'Language', align: 'left', field: 'language' },
  { name: 'updated_at', label: 'Updated at', align: 'left', field: 'updated_at' },
];
const branchColumns = [
  { name: 'name', label: 'name', align: 'left', field: 'name', sortable: true },
  { name: 'sha', label: 'sha', align: 'left', field: 'sha', sortable: true },
  { name: 'url', label: 'URL', align: 'left', field: 'url', sortable: true },
  { name: 'protected', label: 'protected', align: 'left', field: 'protected' },
];
let repoRows = ref([] as repoRowType[]);
let branchRows = ref([] as branchRowDisplayType[]);
onMounted(async () => {
  const res = await axios.get(
    'http://localhost:3000/repositories'
  );
  repoRows.value = [];
  branchRows.value = [];
  const rList = res.data as repoRowType[];
  const resList = rList.map((row) => {
    const mappedRow: repoRowType = {
      id: row.id,
      name: row.name,
      url: row.html_url,
      description: row.description,
      language: row.language,
      updated_at: row.updated_at,
    };
    return mappedRow;
  });
  repoRows.value = resList;
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped></style>
