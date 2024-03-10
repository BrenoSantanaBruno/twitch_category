const AoVivo = ()=>{
    return(
        <div className="
            flex
            flex-row
            justify-between
            items-center
            flex-wrap
            mb-4
        ">
            <div className="mr-4">
                <div className="
                    flex
                    flex-row
                    justify-start
                    items-start
                ">
                    <h1 className="mr-1 uppercase">Ao vivo</h1>
                    <p className="
                        rounded-full
                        bg-[lightgreen]
                        ml-1 w-[1.7rem]
                        h-[1.7rem]
                        border-2
                        border-green-900
                    "></p>
                </div>
                <p>Prestigie quem está ao vivo.</p>
            </div>
            <div className="ml-4">
                <a href="#" className="
                    p-2
                    w-[7rem]
                    h-[2.3rem]
                    bg-indigo-500
                    rounded-[5px]
                ">Estou com sorte</a>
            </div>
        </div>
    );
}

export default AoVivo;
