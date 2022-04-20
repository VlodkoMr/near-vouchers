<template>
  <main class="container">
    <div class="text-center mt-lg-5 mb-lg-2 mt-2">
      <img src="../assets/logo.png" alt="logo" width="200">
    </div>
    <h2 class="mb-lg-5 mb-3 text-center">
      Payment Vouchers
    </h2>
    <div class="landing-top">
      <img src="../assets/sample.jpg" alt="sample" class="sample-image">
      <div>
        <p class="mt-2 pt-3">
          <b>Extend the finance management with NEAR Payment Vouchers!</b>
        </p>
        <p>Lock your NEAR tokens using our smart contract and receive unique voucher that can be used by scan QR-code or follow URL to get locked tokens.</p>
        <p>We support different payment types and allow to receive all tokens or claim tokens by linear unlock.</p>
        <p class="pt-2 text-md-start text-center">
          <button class="btn btn-primary" v-on:click="login">Sign In &raquo;</button>
        </p>
      </div>
    </div>

    <div class="row text-center mt-5">
      <div class="col-10 offset-1 mt-2">
        <div class="row">
          <div class="col-4">
            <div class="counter">1</div>
            <p class="fw-bolder mb-1">Login using your <a href="https://wallet.near.org/" class="text-black" target="_blank">NEAR Wallet</a>.</p>
            <div>NEAR Blockchain - fast and cheap solution for secure transactions.</div>
          </div>
          <div class="col-4">
            <div class="counter">2</div>
            <p class="fw-bolder mb-1">Generate payment vouchers.</p>
            <div>You can create one or multiple Vouchers in few seconds!</div>
          </div>
          <div class="col-4">
            <div class="counter">3</div>
            <p class="fw-bolder mb-1">Send secret URL or scan QR-code.</p>
            <div>Mobile-friendly solution allow to use Payment Vouchers on any device.</div>
          </div>
        </div>
      </div>
    </div>

    <h3 class="text-center mt-5 mb-4">Payment Options</h3>
    <div class="row">
      <div class="col-6 text-right">
        <img src="../assets/unlocked.jpg" alt="" class="sample-image" width="60%">
      </div>
      <div class="col-6">
        <h5 class="mt-2">Unlocked Voucher</h5>
        <p class="mb-1">Useful for:</p>
        <ul>
          <li>Send direct payments.</li>
          <li>Reward for event participants.</li>
          <li>Gift-cards for anyone.</li>
        </ul>
      </div>
    </div>

    <div class="row mt-4">
      <div class="col-6 text-right">
        <img src="../assets/lock.jpg" alt="" class="sample-image" width="60%">
      </div>
      <div class="col-6">
        <h5 class="pt-4">Linear-lock Voucher</h5>
        <p class="mb-1">Useful for:</p>
        <ul>
          <li>Token Vesting.</li>
          <li>Long-term payments.</li>
        </ul>
      </div>
    </div>
    <br>

    <footer>
      NEAR Voucher &copy; 2022
    </footer>
  </main>
</template>

<script>
import { login } from "../utils"

export default {
  name: "SignedOut",
  methods: {
    login() {
      login()
    },
  },
}
</script>

<style>
.landing-top {
  display: grid;
  grid-template-columns: 45% auto;
  grid-gap: 5%;
  font-size: 17px;
}

.landing-top img {
  max-width: 100%;
}

.btn {
  padding: 8px 28px !important;
}

.sample-image {
  border-radius: 15px;
  box-shadow: inset 0 0 10px rgba(0, 0, 0, 0.1);
  padding: 10px;
  background: #efefef;
}

footer {
  position: absolute;
  bottom: 10px;
  left: 0;
  right: 0;
  text-align: center;
  color: #999;
  font-size: 12px;
  border-top: 1px dashed #CCC;
  padding-top: 10px;
}

@media all and (max-width: 640px) {
  .landing-top {
    display: block;
  }
}
</style>
