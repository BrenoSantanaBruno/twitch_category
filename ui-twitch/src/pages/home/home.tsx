import Header from '../../components/Header';
import Footer from '../../components/Footer';
import CardCanal from '../../components/CardCanal';
import AoVivo from '../../components/AoVivo';

const Home = ()=>{
    return(
        <div>
            <Header />
            <AoVivo />
            <CardCanal />
            <Footer />
        </div>
    );
}

export default Home;
