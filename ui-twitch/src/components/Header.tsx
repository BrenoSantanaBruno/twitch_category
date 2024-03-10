import React from 'react';
import logo from '../arquivos/imagens/logoSiteStreamers.png';

const Header = ()=>{
    return(
        <div className="
            home-ui
            flex
            flex-row
            justify-around
            items-center
            flex-wrap
        ">
            <div className="
                logo-ui
                flex
                flex-row
                justify-around
                items-center
                flex-wrap
                ml-2
                mr-[3rem]
            ">
                <p>
                    <a href="#"><img className='
                        w-[8rem]
                    ' src={logo} alt="Logo" /></a>
                </p>
            </div>
            <div className="
                assistir-ui
                header-ui
                flex
                flex-row
                justify-around
                items-center
                flex-wrap
                ml-4
                mr-[3rem]
            ">
                <p className="
                    item-menu
                    ml-2
                    mr-2
                    border-b-4
                  border-indigo-500
                  hover:border-purple-500
                "><a href="#">Assistir</a></p>
                <p className="
                    item-menu
                    ml-2
                    mr-2
                    border-b-4
                  border-indigo-500
                  hover:border-purple-500
                "><a href="#">Sobre</a></p>
            </div>
            <div className="
                github-ui
                header
                flex
                flex-row
                justify-around
                items-center
                flex-wrap
                ml-4
                mr-2
            ">
                <p className="
                    item-menu
                    ml-2
                    mr-2
                    border-b-4
                  border-indigo-500
                  hover:border-purple-500
                "><a href="#">GitHub</a></p>
                <p className="
                    item-menu
                    ml-2
                    mr-2
                    border-b-4
                  border-indigo-500
                  hover:border-purple-500
                "><a href="#">Discord</a></p>
            </div>
            <div className="
                mt-3
                mb-[7rem]
                border-b-2
                border-purple-500
                w-[100%]
            ">

            </div>
        </div>
    );
}

export default Header;
