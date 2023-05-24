import { UseInkProvider } from 'useink';
import { RococoContractsTestnet, ShibuyaTestnet } from 'useink/chains';

function App() {
  return (
    <UseInkProvider 
      config={{ 
        dappName: 'Flipper', 
        chains: [RococoContractsTestnet, ShibuyaTestnet] ,
      }}
    >
      {/* <MyRoutes /> */}
    </UseInkProvider>
  );
}

export default App