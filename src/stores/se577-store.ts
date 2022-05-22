import { defineStore } from 'pinia';
import axios from 'axios';

export type SE577State = {
  course: string;
  instructor: string;
  repos: string[];
  ghID: string;
  ghProxyURL: string;
};

export const useSE577Store = defineStore('SE577Store', {
  state: () =>
    ({
      course: 'se577',
      instructor: 'dr. mitchell',
      repos: [],
      ghID: 'ArchitectingSoftware',
      ghProxyURL: 'http://localhost:4080/gh',
    } as SE577State),
  getters: {
    randomRepo: (state) => {
      const max = state.repos.length - 1;
      const min = 0;
      const randomIndex = Math.floor(Math.random() * (max - min + 1)) + min;
      return state.repos[randomIndex];
    },
    repoCount: (state) => state.repos.length,
  },
  actions: {
    loadTestRepos() {
      this.repos = [];
      this.repos.push('ghrepo1');
      this.repos.push('ghrepos2');
    },
    async loadGHRepos() {
      const proxyURL = `${this.ghProxyURL}/users/architectingsoftware/repos`;
      console.log('proxy url ', proxyURL);
      const result = await axios.get(proxyURL);
      this.repos = [];
      if (result.status === 200) {
        //it worked
        const repoList = result.data;
        console.log('Number of rows returned ', repoList.length);

        for (const repo of repoList) {
          this.repos.push(repo.full_name);
        }
      }
    },
  },
});
