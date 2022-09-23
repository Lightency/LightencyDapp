import styled from 'styled-components'

export const FlexContainer = styled.nav`
  display: flex;
  flex-direction: row;
`

export const NavbarContainer = styled.nav`
  width: 100%;
  height: 40px;
  background-color: black;
  display: flex;
  flex-direction: column;
  opacity: 40%;
  position: absolute;
`

export const RightContainer = styled.div`
  flex: 30%;
  display: flex;
  justify-content: flex-end;
  padding-right: 50px;
  background-color: salamon;
`
export const LeftContainer = styled.div`
  display: none;

  @media only screen and (max-width: 550px) {
    flex: 30%;
    display: flex;
    justify-content: flex-start;
    padding-left: 1rem;
    background-color: salamon;
  }
`

export const ConnectButton = styled.button`
  margin-top: 0.2rem;
  width: 18rem;
  height: 2rem;
  position: relative;
  background-color: #ffde00;
  border-radius: 20%;
  color: black;
  font-weight: bold;
`

export const TranslateButton = styled.button`
  margin-top: 0.2rem;
`
