use sp_std::vec::Vec;

pub fn is_vec_unique<T>(vec: &Vec<T>) -> bool 
    where T: Eq
{
    vec.iter().enumerate().all(|(i, item)| !vec[i+1..].contains(item))
}