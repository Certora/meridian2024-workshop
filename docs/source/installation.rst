Installation
============

Installing Sunbeam
------------------

#. First, we will need to install the Sunbeam Prover.
   For that, please visit `Certora.com <https://www.certora.com/>`_ and sign up for a
   free account `here <https://www.certora.com/signup>`_.
#. You will receive an email with a temporary password and a *Certora Key*.
   Use the password to login to Certora following the link in the email.
#. Next, install Python3.8.16 or newer on your machine.
   If you already have Python3 installed, you can check the version: ``python3 --version``.
   If you need to upgrade, follow the instructions
   `here <https://wiki.python.org/moin/BeginnersGuide/Download>`_.
#. Next, install Java. Check your Java version: ``java -version``.
   If the version is < 11, download and install Java version 11 or later from
   `Oracle <https://www.oracle.com/java/technologies/downloads/>`_.
#. Then, install the Certora Prover: ``pip3 install certora-cli-beta``.

   .. tip:: Always use a Python virtual environment when installing packages.

#. Recall that you received a *Certora Key* in your email (Step 2).
   Use the key to set a temporary environment variable like so
   ``export CERTORAKEY=<personal_access_key>``.
   Alternative, store the key in your profile see
   `here <https://docs.certora.com/en/latest/docs/user-guide/install.html#step-3-set-the-personal-access-key-as-an-environment-variable>`_.


Rust and Stellar CLI Setup
--------------------------

#. We recommend installing Rust as on the
   `official website <https://www.rust-lang.org/tools/install>`_: 
   ``curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``
#. Next, install the WASM target like so: ``rustup target add wasm32-unknown-unknown``
#. We recommend setting up the Stellar CLI as shown
   `here <https://soroban.stellar.org/docs/getting-started/setup#install-the-soroban-cli>`_.
#. We also recommend installing the `wabt <https://github.com/WebAssembly/wabt>`_ toolkit. 
   ``wasm2wat`` is a useful tool for converting the WASM bytecode to a human readable format.
   Ensure that ``wasm2wat`` is executable as a command from your terminal.
   You will have to add ``wabt/bin`` to your ``PATH`` by running
   ``export PATH=~/path/to/wabt/bin:$PATH``. 
#. Finally, install ``rustfilt`` like so: ``cargo install rustfilt``.

----

With that, you should be all set for using Certora Sunbeam. Congratulations!
