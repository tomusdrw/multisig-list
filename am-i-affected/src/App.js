import React, { Component } from 'react'
import numeral from 'numeral'
import { Form, Grid, Header, Message } from 'semantic-ui-react' 
import './App.css'

const wallet2owner = require('./wallet2owner.json');
const owner2wallet = require('./owner2wallet.json');

class App extends Component {
  state = {
    value: '',
  }

  handleChange = ev => {
    const { value } = ev.target
    const { error, message, warn } = checkAddress(value)
    this.setState({ error, message, warn, value })
  }

  render() {
    const { value, message, error, warn } = this.state

    return (
      <Grid
        textAlign='center'
        style={{ height: '100%', marginTop: '25%' }}
        verticalAlign='middle'
      >
        <Grid.Column style={{ maxWidth: 600 }}>
          <Header as='h1' textAlign='center' color='violet'>Parity MultiSig Freeze</Header>
          <Header as='h3' textAlign='center'>Is your address affected?</Header>
          <Form
            size='large'
            error={!!error}
            warning={!!warn}
            success={!!message}
          >
            <Form.Input
              error={!!error}
              fluid
              icon='address card'
              iconPosition='left'
              placeholder='Your Ethereum Address'
              onChange={this.handleChange}
              value={value}
            />
            <Message warning>
              {warn}
            </Message>
            <Message error>
              {error}
            </Message>
            <Message success>
              {message}
            </Message>
          </Form>
          <Message>
            <Grid>
              <Grid.Row>
                <Grid.Column width={8}>
                  <p>Affected wallets: <strong>{numeral(Object.keys(wallet2owner).length).format()}</strong></p>
                </Grid.Column>
                <Grid.Column width={8}>
                  <p>Affected owners: <strong>{numeral(Object.keys(owner2wallet).length).format()}</strong></p>
                </Grid.Column>
              </Grid.Row>
            </Grid>
          </Message>
        </Grid.Column>
      </Grid>
    );
  }
}

export default App;


function checkAddress (address) {
  if (!address.startsWith('0x')) {
    return checkAddress('0x' + address);
  }

  if (!/^[0-9a-f]+$/.test(address.substr(2).toLowerCase())) {
    return {
      error: 'The address contains invalid characters.'
    }
  }

  if (address.length !== 42) {
    return {
      error: 'This does not look like correct Ethereum address.'
    }
  }

  if (owner2wallet[address]) {
    const wallets = owner2wallet[address]
    return {
      warn: `You are affected. Your wallet ${wallets.join(', ')} is frozen.`
    }
  }

  if (wallet2owner[address]) {
    const owners = wallet2owner[address].length
    return {
      warn: `You are affected. This address is a Parity Multisig wallet with ${owners} owners.`
    }
  }

  return {
    message: 'Your address is safe. It is not a multisig wallet nor an owner of one.'
  }
}
