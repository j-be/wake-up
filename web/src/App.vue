<template>
  <div class="container">
    <DeviceList></DeviceList>
  </div>
</template>

<script lang="ts">
import {Options, Vue} from 'vue-class-component';
import DeviceList from '@/components/Device.vue';

@Options({
  components: {
    DeviceList,
  },
  methods: {
    methods: {
      awake(name: string): void {
        console.log(name);
      }
    },
  }
})
export default class App extends Vue {}
</script>
<style lang="sass">
body
  background-color: #fff
</style>
