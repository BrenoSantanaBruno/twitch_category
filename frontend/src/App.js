import React from 'react';
import axios from 'axios';

function TwitchStreams() {
    const [streams, setStreams] = React.useState([]);

    React.useEffect(() => {
        const fetchData = async () => {
            try {
                const response = await axios.get('http://localhost:8080/api/twitch-streams');
                setStreams(response.data.data);
            } catch (error) {
                console.error('Erro ao buscar os dados: ', error);
            }
        };
        fetchData();
    }, []);

    return (
        <div>
            <h1 className="title">Twitch Streams</h1>
            <div className="stream-container">
                {streams.map((stream, index) => (
                    <a key={stream.user_id} href={`https://www.twitch.tv/${stream.user_name}`} target="_blank" rel="noopener noreferrer" className="stream-card" style={{ animationDelay: `${index * 0.1}s` }}>
                        <img src={stream.thumbnail_url.replace('{width}', '300').replace('{height}', '300')} alt={stream.user_name} />
                        <div className="stream-details">
                            <h2>{stream.user_name}</h2>
                            <p>Título: {stream.title}</p>
                            <p>Categoria: {stream.game_name}</p>
                            <p>Viewers: {stream.viewer_count}</p>
                        </div>
                    </a>
                ))}
            </div>
        </div>
    );
}

function App() {
    return (
        <div className="App">
            <TwitchStreams />
        </div>
    );
}

export default App;
