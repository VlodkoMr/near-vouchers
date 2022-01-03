<template>
  <header class="pt-3 mb-4">
    <div class="container-lg d-flex justify-content-between">
      <img src="../assets/logo.png" width="150" alt="logos">
      <div>
        <span class="username">{{ accountId }}</span>
        <button class="btn btn-primary" v-on:click="logout">Sign out</button>
      </div>
    </div>
  </header>
</template>

<script>
import {logout} from '../utils';

export default {
  name: "Header",
  computed: {
    isSignedIn() {
      return window.walletConnection ? window.walletConnection.isSignedIn() : false
    },
    accountId() {
      return window.accountId
    },
  },
  methods: {
    logout: logout,
  }
}
</script>