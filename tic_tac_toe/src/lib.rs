pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {

    for vecteur in &table{
        for pion in vecteur{
            if diagonals(pion, &table) || horizontal(pion, &table) || vertical(pion, &table){
                return format!("player {pion} won")
            }
        }
    }
    return "tie".to_string();
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    if (table[0][0] == player && table[1][1] == player && table[2][2] == player) || (table[0][2] == player && table[2][0] == player && table[1][1] == player){
        return true
    }
    return false;
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    if (table[0][0] == player && table[1][0] == player && table[2][0] == player) || (table[0][1] == player && table[1][1] == player && table[2][1] == player) || (table[0][2] == player && table[1][2] == player && table[2][2] == player){
        return true;
    }
    return false;
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    if (table[0][0] == player && table[0][1] == player && table[0][2] == player) || (table[1][0] == player && table[1][1] == player && table[1][2] == player) || (table[2][0] == player && table[2][1] == player && table[2][2] == player){
        return true;
    }
    return false;
}
